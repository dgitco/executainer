use crate::result_parser::parse_lane_result;
use serde::Serialize;
use std::process::Command;

#[derive(Debug, Clone, Serialize)]
pub struct DoctorCheck {
    pub name: String,
    pub ok: bool,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DoctorReport {
    pub ok: bool,
    pub checks: Vec<DoctorCheck>,
}

pub trait DoctorRunner {
    fn command_exists(&self, command: &str) -> bool;
    fn writable_temp_dir(&self) -> bool;
    fn parser_self_check(&self) -> bool;
}

pub struct SystemDoctorRunner;

impl DoctorRunner for SystemDoctorRunner {
    fn command_exists(&self, command: &str) -> bool {
        Command::new("sh")
            .arg("-lc")
            .arg(format!("command -v {command} >/dev/null 2>&1"))
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }

    fn writable_temp_dir(&self) -> bool {
        let path = std::env::temp_dir().join(format!("executainer-doctor-{}", std::process::id()));
        let result = std::fs::write(&path, "ok").is_ok();
        let _ = std::fs::remove_file(path);
        result
    }

    fn parser_self_check(&self) -> bool {
        let sample = r#"
<<<EXECUTAINER_RESULT_START>>>
{"lane_id":"lane-01","status":"ok","summary":"summary","proposed_files":[],"deferred_files":[],"notes":"none"}
<<<EXECUTAINER_RESULT_END>>>
"#;
        parse_lane_result(sample).is_ok()
    }
}

pub fn run_doctor(runner: &impl DoctorRunner) -> DoctorReport {
    let runtime_ok = std::env::current_exe().is_ok();
    let checks = vec![
        DoctorCheck {
            name: "runtime".into(),
            ok: runtime_ok,
            detail: if runtime_ok {
                "current executable resolved".into()
            } else {
                "could not resolve current executable".into()
            },
        },
        DoctorCheck {
            name: "tmux".into(),
            ok: runner.command_exists("tmux"),
            detail: "tmux must be installed for the v1 backend".into(),
        },
        DoctorCheck {
            name: "codex_cli".into(),
            ok: runner.command_exists("codex"),
            detail: "codex CLI must be available on PATH".into(),
        },
        DoctorCheck {
            name: "writable_temp_dir".into(),
            ok: runner.writable_temp_dir(),
            detail: "tmp/executainer must be writable".into(),
        },
        DoctorCheck {
            name: "parser_self_check".into(),
            ok: runner.parser_self_check(),
            detail: "sentinel parser must fail closed and parse the golden payload".into(),
        },
    ];
    let ok = checks.iter().all(|check| check.ok);
    DoctorReport { ok, checks }
}
