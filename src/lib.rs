//! Safe, allowlisted execution for staged capability-stop drills.
//!
//! ```
//! use kill_switch_drill::{parse_config, validate};
//! let config = parse_config(include_str!("../examples/kill-switch.toml")).unwrap();
//! validate(&config).unwrap();
//! ```

use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    io::Read,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use thiserror::Error;

const DEFAULT_TIMEOUT_SECONDS: u64 = 30;
const MAX_TIMEOUT_SECONDS: u64 = 30;
const MAX_CAPTURED_STDOUT_BYTES: usize = 1024 * 1024;

#[derive(Debug, Error)]
pub enum DrillError {
    #[error("could not parse configuration: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("configuration error: {0}")]
    Config(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub version: u32,
    #[serde(default)]
    pub allowlist: BTreeMap<String, AllowedCommand>,
    #[serde(default)]
    pub profiles: BTreeMap<String, Profile>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct AllowedCommand {
    pub command: Vec<String>,
    pub expect_stdout: Option<String>,
    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: u64,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Profile {
    pub description: String,
    #[serde(default)]
    pub steps: Vec<Step>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Step {
    pub name: String,
    pub action: String,
    pub verify: Option<String>,
}

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Simulated,
    Passed,
    Failed,
    Skipped,
}
#[derive(Debug, Serialize, Clone)]
pub struct Checkpoint {
    pub name: String,
    pub action_id: String,
    pub action: State,
    pub verification_id: Option<String>,
    pub verification: State,
    pub duration_ms: u128,
    pub note: String,
}
#[derive(Debug, Serialize, Clone)]
pub struct IncidentCard {
    pub schema: &'static str,
    pub profile: String,
    pub description: String,
    pub mode: &'static str,
    pub started_at_unix: u64,
    pub completed_at_unix: u64,
    pub all_confirmed: bool,
    pub checkpoints: Vec<Checkpoint>,
    pub report_safety: &'static str,
}

fn default_timeout_seconds() -> u64 {
    DEFAULT_TIMEOUT_SECONDS
}

pub fn parse_config(source: &str) -> Result<Config, DrillError> {
    Ok(toml::from_str(source)?)
}
pub fn validate(config: &Config) -> Result<(), DrillError> {
    if config.version != 1 {
        return Err(DrillError::Config("only version = 1 is supported".into()));
    }
    if config.allowlist.is_empty() {
        return Err(DrillError::Config("allowlist must not be empty".into()));
    }
    if config.profiles.is_empty() {
        return Err(DrillError::Config("profiles must not be empty".into()));
    }
    for (id, command) in &config.allowlist {
        if id.trim().is_empty() || command.command.first().is_none_or(|c| c.trim().is_empty()) {
            return Err(DrillError::Config(format!(
                "allowlist entry `{id}` needs a non-empty command array"
            )));
        }
        if !(1..=MAX_TIMEOUT_SECONDS).contains(&command.timeout_seconds) {
            return Err(DrillError::Config(format!(
                "allowlist entry `{id}` needs timeout_seconds from 1 to {MAX_TIMEOUT_SECONDS}"
            )));
        }
    }
    for (profile_id, profile) in &config.profiles {
        if profile.steps.is_empty() {
            return Err(DrillError::Config(format!(
                "profile `{profile_id}` needs at least one step"
            )));
        }
        for step in &profile.steps {
            if step.name.trim().is_empty() {
                return Err(DrillError::Config(format!(
                    "profile `{profile_id}` has an unnamed step"
                )));
            }
            if !config.allowlist.contains_key(&step.action) {
                return Err(DrillError::Config(format!(
                    "step `{}` references unknown action `{}`",
                    step.name, step.action
                )));
            }
            if let Some(verify) = &step.verify {
                if !config.allowlist.contains_key(verify) {
                    return Err(DrillError::Config(format!(
                        "step `{}` references unknown verification `{verify}`",
                        step.name
                    )));
                }
            }
        }
    }
    Ok(())
}

fn read_capped_stdout(mut stdout: std::process::ChildStdout) -> std::io::Result<Vec<u8>> {
    let mut captured = Vec::new();
    let mut buffer = [0; 8192];
    loop {
        let read = stdout.read(&mut buffer)?;
        if read == 0 {
            return Ok(captured);
        }
        let remaining = MAX_CAPTURED_STDOUT_BYTES.saturating_sub(captured.len());
        captured.extend_from_slice(&buffer[..read.min(remaining)]);
    }
}

fn run_allowed(id: &str, command: &AllowedCommand) -> (bool, String) {
    let mut child = match Command::new(&command.command[0])
        .args(&command.command[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            return (
                false,
                format!("could not execute declared command `{id}`: {error}"),
            )
        }
    };
    let stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => {
            return (
                false,
                format!("could not read declared command `{id}` response"),
            )
        }
    };
    let reader = thread::spawn(move || read_capped_stdout(stdout));
    let started = Instant::now();
    let timeout = Duration::from_secs(command.timeout_seconds);
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if started.elapsed() >= timeout => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = reader.join();
                return (
                    false,
                    format!(
                        "declared command `{id}` timed out after {} seconds",
                        command.timeout_seconds
                    ),
                );
            }
            Ok(None) => thread::sleep(Duration::from_millis(10)),
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = reader.join();
                return (
                    false,
                    format!("could not wait for declared command `{id}`: {error}"),
                );
            }
        }
    };
    let stdout = match reader.join() {
        Ok(Ok(stdout)) => stdout,
        Ok(Err(error)) => {
            return (
                false,
                format!("could not read declared command `{id}` response: {error}"),
            )
        }
        Err(_) => {
            return (
                false,
                format!("could not read declared command `{id}` response"),
            )
        }
    };
    let stdout = String::from_utf8_lossy(&stdout);
    let expected = command
        .expect_stdout
        .as_ref()
        .map(|wanted| stdout.contains(wanted))
        .unwrap_or(true);
    let ok = status.success() && expected;
    let note = if status.success() && !expected {
        "command completed but expected response was absent"
    } else if status.success() {
        "control-plane response matched"
    } else {
        "command exited unsuccessfully"
    };
    (ok, note.into())
}
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
pub fn run_drill(
    config: &Config,
    profile_id: &str,
    live: bool,
    verify: bool,
) -> Result<IncidentCard, DrillError> {
    validate(config)?;
    let profile = config
        .profiles
        .get(profile_id)
        .ok_or_else(|| DrillError::Config(format!("profile `{profile_id}` was not found")))?;
    let started = now();
    let mut checkpoints = Vec::new();
    for step in &profile.steps {
        let timer = Instant::now();
        let (action, action_note) = if live {
            let (ok, note) = run_allowed(&step.action, &config.allowlist[&step.action]);
            (if ok { State::Passed } else { State::Failed }, note)
        } else {
            (
                State::Simulated,
                "dry run: allowlisted action was not executed".into(),
            )
        };
        let (verification, note) = match (&step.verify, verify) {
            (None, _) => (
                State::Skipped,
                format!("{action_note}; no verification command declared"),
            ),
            (Some(_), false) => (
                State::Skipped,
                format!("{action_note}; verification disabled by operator"),
            ),
            (Some(id), true) => {
                let (ok, check_note) = run_allowed(id, &config.allowlist[id]);
                (
                    if ok { State::Passed } else { State::Failed },
                    format!("{action_note}; {check_note}"),
                )
            }
        };
        checkpoints.push(Checkpoint {
            name: step.name.clone(),
            action_id: step.action.clone(),
            action,
            verification_id: step.verify.clone(),
            verification,
            duration_ms: timer.elapsed().as_millis(),
            note,
        });
    }
    let all_confirmed = checkpoints.iter().all(|c| {
        c.verification == State::Passed
            && if live {
                c.action == State::Passed
            } else {
                c.action == State::Simulated
            }
    });
    Ok(IncidentCard { schema: "agent-kill-switch-drill/incident-card@v1", profile: profile_id.into(), description: profile.description.clone(), mode: if live { "live" } else { "dry_run" }, started_at_unix: started, completed_at_unix: now(), all_confirmed, checkpoints, report_safety: "Command IDs and statuses only. Command lines, output, environment, and provider secrets are never included." })
}
pub fn write_card(path: &std::path::Path, card: &IncidentCard) -> Result<(), std::io::Error> {
    std::fs::write(
        path,
        serde_json::to_string_pretty(card).expect("serializable") + "\n",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn documented_example_validates() {
        let c = parse_config(include_str!("../examples/kill-switch.toml")).unwrap();
        validate(&c).unwrap();
    }
    #[test]
    fn rejects_unknown_command() {
        let s = "version=1\n[allowlist.a]\ncommand=[\"true\"]\n[profiles.p]\ndescription=\"x\"\n[[profiles.p.steps]]\nname=\"x\"\naction=\"nope\"\n";
        assert!(validate(&parse_config(s).unwrap()).is_err());
    }
    #[test]
    fn dry_run_executes_check_not_action() {
        let s = "version=1\n[allowlist.act]\ncommand=[\"false\"]\n[allowlist.check]\ncommand=[\"true\"]\n[profiles.p]\ndescription=\"x\"\n[[profiles.p.steps]]\nname=\"x\"\naction=\"act\"\nverify=\"check\"\n";
        let c = run_drill(&parse_config(s).unwrap(), "p", false, true).unwrap();
        assert!(c.all_confirmed);
        assert_eq!(c.checkpoints[0].action, State::Simulated);
    }
    #[test]
    fn live_failed_action_prevents_confirmation() {
        let s = "version=1\n[allowlist.act]\ncommand=[\"false\"]\n[allowlist.check]\ncommand=[\"true\"]\n[profiles.p]\ndescription=\"x\"\n[[profiles.p.steps]]\nname=\"x\"\naction=\"act\"\nverify=\"check\"\n";
        let c = run_drill(&parse_config(s).unwrap(), "p", true, true).unwrap();
        assert!(!c.all_confirmed);
        assert_eq!(c.checkpoints[0].action, State::Failed);
    }
    #[test]
    fn timed_out_verification_becomes_a_failed_checkpoint() {
        let s = "version=1\n[allowlist.act]\ncommand=[\"true\"]\n[allowlist.check]\ncommand=[\"sleep\", \"2\"]\ntimeout_seconds=1\n[profiles.p]\ndescription=\"x\"\n[[profiles.p.steps]]\nname=\"x\"\naction=\"act\"\nverify=\"check\"\n";
        let card = run_drill(&parse_config(s).unwrap(), "p", false, true).unwrap();
        assert!(!card.all_confirmed);
        assert_eq!(card.checkpoints[0].verification, State::Failed);
        assert!(card.checkpoints[0]
            .note
            .contains("timed out after 1 seconds"));
    }
    #[test]
    fn reports_exclude_command_details_and_output() {
        let secret = "PROVIDER_SECRET=not-for-the-card";
        let s = format!(
            "version=1\n[allowlist.act]\ncommand=[\"true\"]\n[allowlist.check]\ncommand=[\"printf\", \"{secret}\"]\n[profiles.p]\ndescription=\"x\"\n[[profiles.p.steps]]\nname=\"x\"\naction=\"act\"\nverify=\"check\"\n"
        );
        let card = run_drill(&parse_config(&s).unwrap(), "p", false, true).unwrap();
        let report = serde_json::to_string(&card).unwrap();
        assert!(!report.contains(secret));
        assert!(!report.contains("printf"));
    }
}
