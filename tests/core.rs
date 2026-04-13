use executainer::doctor::{DoctorRunner, run_doctor};
use executainer::manifest::{ConflictStatus, LaneMode, ParseStatus, RunManifest, SynthesisStatus};
use executainer::prompt_builder::{LanePromptInput, build_lane_prompt};
use executainer::result_parser::{ParsedLaneResult, parse_lane_result};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[test]
fn parses_valid_sentinel_block() {
    let raw = r#"
noise
<<<EXECUTAINER_RESULT_START>>>
{"lane_id":"lane-01","status":"ok","summary":"done","proposed_files":["src/main.rs"],"deferred_files":["README.md"],"notes":"ship it"}
<<<EXECUTAINER_RESULT_END>>>
"#;

    let parsed = parse_lane_result(raw).expect("sentinel block should parse");
    assert_eq!(
        parsed,
        ParsedLaneResult {
            lane_id: "lane-01".into(),
            status: "ok".into(),
            summary: "done".into(),
            proposed_files: vec!["src/main.rs".into()],
            deferred_files: vec!["README.md".into()],
            notes: "ship it".into(),
        }
    );
}

#[test]
fn fails_closed_when_sentinel_missing() {
    let raw = r#"{"lane_id":"lane-01"}"#;
    assert!(parse_lane_result(raw).is_err());
}

#[test]
fn lane_prompt_records_deferred_files_and_read_only_contract() {
    let prompt = build_lane_prompt(LanePromptInput {
        lane_id: "lane-01",
        mode: LaneMode::ReadOnly,
        task: "Audit the repository",
        deferred_files: &["README.md".into(), "docs/ADR.md".into()],
        writable_scope: None,
    });

    assert!(prompt.contains("lane-01"));
    assert!(prompt.contains("read-only"));
    assert!(prompt.contains("README.md"));
    assert!(prompt.contains("<<<EXECUTAINER_RESULT_START>>>"));
}

#[test]
fn manifest_serializes_with_deterministic_keys() {
    let manifest = RunManifest {
        run_slug: "20260413-123000-abc123".into(),
        created_at: "2026-04-13T12:30:00Z".into(),
        cwd: PathBuf::from("/tmp/work"),
        requested_lane_count: 3,
        lane_ids: vec!["lane-01".into(), "lane-02".into(), "lane-03".into()],
        lane_modes: BTreeMap::from([
            ("lane-01".into(), LaneMode::ReadOnly),
            ("lane-02".into(), LaneMode::ReadOnly),
            ("lane-03".into(), LaneMode::ReadOnly),
        ]),
        writable_scopes: BTreeMap::new(),
        prompt_template_refs: BTreeMap::from([(
            "lane-01".into(),
            "templates/lane_read_only.txt".into(),
        )]),
        deferred_files: vec!["README.md".into()],
        capture_files: BTreeMap::new(),
        parse_status: ParseStatus::Pending,
        synthesis_status: SynthesisStatus::Pending,
        conflict_status: ConflictStatus::Clear,
        session_name: Some("executainer-20260413-123000-abc123".into()),
        approval_events: vec![],
        notes: vec![],
    };

    let json = manifest.to_json().expect("manifest should serialize");
    assert!(json.contains("\"run_slug\""));
    assert!(json.contains("\"lane_modes\""));
}

struct StubDoctorRunner {
    tmux: bool,
    codex: bool,
    parser_ok: bool,
    temp_ok: bool,
}

impl DoctorRunner for StubDoctorRunner {
    fn command_exists(&self, command: &str) -> bool {
        match command {
            "tmux" => self.tmux,
            "codex" => self.codex,
            _ => false,
        }
    }

    fn writable_temp_dir(&self) -> bool {
        self.temp_ok
    }

    fn parser_self_check(&self) -> bool {
        self.parser_ok
    }
}

#[test]
fn doctor_reports_failing_checks() {
    let report = run_doctor(&StubDoctorRunner {
        tmux: true,
        codex: false,
        parser_ok: true,
        temp_ok: true,
    });

    assert!(!report.ok);
    assert_eq!(report.checks.len(), 5);
    assert!(
        report
            .checks
            .iter()
            .any(|check| check.name == "codex_cli" && !check.ok)
    );
}
