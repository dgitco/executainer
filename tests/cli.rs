use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn doctor_json_outputs_machine_readable_payload() {
    Command::cargo_bin("executainer")
        .expect("binary exists")
        .arg("doctor")
        .arg("--json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"checks\""));
}

#[test]
fn inspect_missing_run_returns_error() {
    Command::cargo_bin("executainer")
        .expect("binary exists")
        .arg("inspect")
        .arg("missing-run")
        .assert()
        .failure()
        .stderr(predicate::str::contains("missing-run"));
}

#[test]
fn clean_requires_yes_flag() {
    Command::cargo_bin("executainer")
        .expect("binary exists")
        .arg("clean")
        .arg("missing-run")
        .assert()
        .failure()
        .stderr(predicate::str::contains("clean requires --yes"));
}

#[test]
fn run_rejects_zero_lanes_before_preflight() {
    Command::cargo_bin("executainer")
        .expect("binary exists")
        .args(["run", "--lanes", "0", "--task", "Audit this repository"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--lanes must be at least 1"));
}

#[test]
fn run_rejects_conflicting_task_inputs() {
    let temp = tempfile::NamedTempFile::new().expect("temp file");

    Command::cargo_bin("executainer")
        .expect("binary exists")
        .args([
            "run",
            "--task",
            "Audit this repository",
            "--task-file",
            temp.path().to_str().expect("utf-8 temp path"),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "choose either --task or --task-file",
        ));
}
