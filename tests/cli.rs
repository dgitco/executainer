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
