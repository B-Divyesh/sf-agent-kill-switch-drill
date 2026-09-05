use clap::{Parser, Subcommand};
use kill_switch_drill::{parse_config, run_drill, validate, write_card, DrillError, IncidentCard};
use std::{
    fs,
    path::PathBuf,
    process::{self, ExitCode},
    time::{SystemTime, UNIX_EPOCH},
};
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
    /// Run the bundled harmless sample and write its incident card to a temp directory.
    Demo {
        #[arg(long)]
        json: bool,
    },
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

fn print_card(card: &IncidentCard, json: bool) {
    if json {
        println!("{}", serde_json::to_string_pretty(card).unwrap());
    } else {
        println!(
            "INCIDENT CARD · {} · {}",
            card.profile,
            card.mode.replace('_', " ")
        );
        for checkpoint in &card.checkpoints {
            println!(
                "{} — action: {:?}; verification: {:?} ({})",
                checkpoint.name, checkpoint.action, checkpoint.verification, checkpoint.note
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
}

fn demo_directory() -> Result<PathBuf, String> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();
    let directory = std::env::temp_dir().join(format!(
        "agent-kill-switch-drill-demo-{}-{nonce}",
        process::id()
    ));
    fs::create_dir(&directory)
        .map_err(|error| format!("could not create demo directory: {error}"))?;
    Ok(directory)
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
        Commands::Demo { json } => {
            let directory = demo_directory()?;
            let config = directory.join("kill-switch.toml");
            let report = directory.join("incident-card.json");
            fs::write(&config, SAMPLE).map_err(|error| error.to_string())?;
            let config = load(&config)?;
            let card = run_drill(&config, "sample", false, true)
                .map_err(|error: DrillError| error.to_string())?;
            write_card(&report, &card)
                .map_err(|error| format!("could not write demo report: {error}"))?;
            eprintln!("Demo report: {}", report.display());
            print_card(&card, json);
            Ok(if card.all_confirmed { 0 } else { 2 })
        }
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
            print_card(&card, json);
            Ok(if card.all_confirmed { 0 } else { 2 })
        }
    }
}
