use std::{fs, process::Command};

#[test]
fn demo_runs_the_bundled_sample_and_writes_a_temp_report() {
    let output = Command::new(env!("CARGO_BIN_EXE_agent-kill-switch-drill"))
        .args(["demo", "--json"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let card: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(card["profile"], "sample");
    assert_eq!(card["all_confirmed"], true);
    let report = String::from_utf8(output.stderr).unwrap();
    let path = std::path::PathBuf::from(report.trim().strip_prefix("Demo report: ").unwrap());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&fs::read(&path).unwrap()).unwrap()["profile"],
        "sample"
    );
    fs::remove_dir_all(path.parent().unwrap()).unwrap();
}

#[test]
fn timed_out_verification_writes_a_reviewable_report() {
    let directory = tempfile::tempdir().unwrap();
    let config = directory.path().join("kill-switch.toml");
    let report = directory.path().join("incident-card.json");
    fs::write(
        &config,
        "version=1\n[allowlist.action]\ncommand=[\"true\"]\n[allowlist.check]\ncommand=[\"sleep\", \"2\"]\ntimeout_seconds=1\n[profiles.sample]\ndescription=\"Timeout report\"\n[[profiles.sample.steps]]\nname=\"Check the control plane\"\naction=\"action\"\nverify=\"check\"\n",
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_agent-kill-switch-drill"))
        .args([
            "drill",
            "sample",
            "--config",
            config.to_str().unwrap(),
            "--report",
            report.to_str().unwrap(),
            "--json",
        ])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    let card: serde_json::Value = serde_json::from_slice(&fs::read(&report).unwrap()).unwrap();
    assert_eq!(card["all_confirmed"], false);
    assert_eq!(card["checkpoints"][0]["verification"], "failed");
    assert!(card["checkpoints"][0]["note"]
        .as_str()
        .unwrap()
        .contains("timed out after 1 seconds"));
}
