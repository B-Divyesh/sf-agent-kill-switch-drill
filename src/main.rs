use clap::{Parser, Subcommand};
use kill_switch_drill::{parse_config, run_drill, validate, write_card, DrillError};
use std::{fs, path::PathBuf, process::ExitCode};
const SAMPLE: &str = include_str!("../examples/kill-switch.toml");
#[derive(Parser)]
#[command(
    name = "agent-kill-switch-drill",
    version,
    about = "Rehearse an allowlisted per-capability stop path.",
    long_about = "Dry-run is the default: action commands are simulated while declared verification commands run. Live actions require both --live and an exact --confirm PROFILE. Reports omit command lines, output, environment, and provider secrets."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Write a reviewed, harmless sample configuration.
    Init {
        #[arg(default_value = "kill-switch.toml")]
        path: PathBuf,
    },
    /// Check command references and profile structure without running anything.
    Validate {
        #[arg(short, long, default_value = "kill-switch.toml")]
        config: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// Run one staged stop-path rehearsal and optionally save an incident card.
    Drill {
        profile: String,
        #[arg(short, long, default_value = "kill-switch.toml")]
        config: PathBuf,
        #[arg(long)]
        live: bool,
        #[arg(long, value_name = "PROFILE")]
        confirm: Option<String>,
        #[arg(long)]
        no_verify: bool,
        #[arg(long)]
        report: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
}
fn load(path: &PathBuf) -> Result<kill_switch_drill::Config, String> {
    fs::read_to_string(path)
        .map_err(|e| format!("could not read {}: {e}", path.display()))
        .and_then(|s| parse_config(&s).map_err(|e| e.to_string()))
}
fn main() -> ExitCode {
    match execute(Cli::parse()) {
        Ok(code) => ExitCode::from(code),
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::from(1)
        }
    }
}
fn execute(cli: Cli) -> Result<u8, String> {
    match cli.command {
        Commands::Init { path } => {
            if path.exists() {
                return Err(format!(
                    "{} already exists; refusing to overwrite it",
                    path.display()
                ));
            }
            fs::write(&path, SAMPLE).map_err(|e| e.to_string())?;
            println!("Wrote {}. Review every command before use.", path.display());
            Ok(0)
        }
        Commands::Validate { config, json } => {
            let c = load(&config)?;
            validate(&c).map_err(|e| e.to_string())?;
            if json {
                println!(
                    "{{\"valid\":true,\"profiles\":{},\"allowlist\":{}}}",
                    c.profiles.len(),
                    c.allowlist.len()
                );
            } else {
                println!(
                    "Valid: {} profile(s), {} allowlisted command(s).",
                    c.profiles.len(),
                    c.allowlist.len()
                );
            }
            Ok(0)
        }
        Commands::Drill {
            profile,
            config,
            live,
            confirm,
            no_verify,
            report,
            json,
        } => {
            if live && confirm.as_deref() != Some(profile.as_str()) {
                return Err(format!(
                    "live actions are locked. Re-run with --live --confirm {profile}"
                ));
            }
            let c = load(&config)?;
            let card =
                run_drill(&c, &profile, live, !no_verify).map_err(|e: DrillError| e.to_string())?;
            if let Some(path) = report {
                write_card(&path, &card).map_err(|e| format!("could not write report: {e}"))?;
            }
            if json {
                println!("{}", serde_json::to_string_pretty(&card).unwrap());
            } else {
                println!(
                    "INCIDENT CARD · {} · {}",
                    card.profile,
                    card.mode.replace('_', " ")
                );
                for c in &card.checkpoints {
                    println!(
                        "{} — action: {:?}; verification: {:?} ({})",
                        c.name, c.action, c.verification, c.note
                    );
                }
                println!(
                    "RESULT: {}",
                    if card.all_confirmed {
                        "ALL DECLARED PATHS CONFIRMED"
                    } else {
                        "REVIEW REQUIRED"
                    }
                );
            }
            Ok(if card.all_confirmed { 0 } else { 2 })
        }
    }
}
