use crate::doctor::{SystemDoctorRunner, run_doctor};
use crate::manifest::{ConflictStatus, LaneMode, ParseStatus, RunManifest, SynthesisStatus};
use crate::prompt_builder::{LanePromptInput, build_lane_prompt, build_synthesis_prompt};
use crate::result_parser::parse_lane_result;
use crate::synthesizer::synthesize_results;
use crate::tmux_backend;
use chrono::Utc;
use clap::{Args, Parser, Subcommand};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct AppError {
    exit_code: i32,
    message: String,
}

impl AppError {
    fn new(exit_code: i32, message: impl Into<String>) -> Self {
        Self {
            exit_code,
            message: message.into(),
        }
    }

    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {}

#[derive(Debug, Parser)]
#[command(name = "executainer", about = "CLI-first trusted parallel delegation")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Doctor(DoctorCommand),
    Run(RunCommand),
    Inspect(InspectCommand),
    Clean(CleanCommand),
}

#[derive(Debug, Args)]
struct DoctorCommand {
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Args)]
struct RunCommand {
    #[arg(long, default_value_t = 3)]
    lanes: usize,
    #[arg(long)]
    task: Option<String>,
    #[arg(long = "task-file")]
    task_file: Option<PathBuf>,
    #[arg(long = "defer-file")]
    defer_file: Vec<String>,
    #[arg(long = "writable-lane")]
    writable_lane: Vec<String>,
}

#[derive(Debug, Args)]
struct InspectCommand {
    run_slug: String,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Args)]
struct CleanCommand {
    run_slug: String,
    #[arg(long)]
    yes: bool,
}

pub fn run_app() -> Result<(), AppError> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Doctor(args) => doctor_command(args),
        Commands::Run(args) => run_command(args),
        Commands::Inspect(args) => inspect_command(args),
        Commands::Clean(args) => clean_command(args),
    }
}

fn doctor_command(args: DoctorCommand) -> Result<(), AppError> {
    let report = run_doctor(&SystemDoctorRunner);
    if args.json {
        let output = serde_json::to_string_pretty(&report)
            .map_err(|err| AppError::new(1, format!("failed to serialize doctor report: {err}")))?;
        println!("{output}");
    } else {
        println!("executainer doctor");
        for check in &report.checks {
            let status = if check.ok { "OK" } else { "FAIL" };
            println!("- {:<16} {:<4} {}", check.name, status, check.detail);
        }
        println!();
        println!("overall: {}", if report.ok { "ready" } else { "not ready" });
    }

    if report.ok {
        Ok(())
    } else {
        Err(AppError::new(1, "doctor checks failed"))
    }
}

fn run_command(args: RunCommand) -> Result<(), AppError> {
    if args.lanes == 0 {
        return Err(AppError::new(1, "--lanes must be at least 1"));
    }

    let task = resolve_task(args.task, args.task_file)?;
    let doctor = run_doctor(&SystemDoctorRunner);
    if !doctor.ok {
        return Err(AppError::new(1, "doctor preflight failed"));
    }

    let cwd = std::env::current_dir()
        .map_err(|err| AppError::new(1, format!("failed to read cwd: {err}")))?;
    let run_slug = build_run_slug();
    let run_root = cwd.join("tmp").join("executainer").join(&run_slug);
    let session_name = format!("executainer-{run_slug}");
    create_run_dirs(&run_root)?;

    let writable_scopes = parse_writable_lanes(&args.writable_lane, args.lanes)?;
    let lane_ids = (1..=args.lanes)
        .map(|idx| format!("lane-{idx:02}"))
        .collect::<Vec<_>>();
    let lane_modes = lane_ids
        .iter()
        .map(|lane_id| {
            let mode = if writable_scopes.contains_key(lane_id) {
                LaneMode::Writable
            } else {
                LaneMode::ReadOnly
            };
            (lane_id.clone(), mode)
        })
        .collect::<BTreeMap<_, _>>();

    let prompt_template_refs = lane_ids
        .iter()
        .map(|lane_id| {
            let template = match lane_modes.get(lane_id) {
                Some(LaneMode::Writable) => "templates/lane_writable.txt",
                _ => "templates/lane_read_only.txt",
            };
            (lane_id.clone(), template.to_string())
        })
        .collect::<BTreeMap<_, _>>();

    let mut manifest = RunManifest {
        run_slug: run_slug.clone(),
        created_at: Utc::now().to_rfc3339(),
        cwd: cwd.clone(),
        requested_lane_count: args.lanes,
        lane_ids: lane_ids.clone(),
        lane_modes,
        writable_scopes,
        prompt_template_refs,
        deferred_files: args.defer_file.clone(),
        capture_files: BTreeMap::new(),
        parse_status: ParseStatus::Pending,
        synthesis_status: SynthesisStatus::Pending,
        conflict_status: ConflictStatus::Clear,
        session_name: Some(session_name.clone()),
        approval_events: vec![],
        notes: vec![],
    };

    let inputs_dir = run_root.join("inputs");
    let prompts_dir = run_root.join("prompts");
    let captures_dir = run_root.join("captures");
    let outputs_dir = run_root.join("outputs");
    let manifest_path = run_root.join("manifest.json");

    for lane_id in &lane_ids {
        let prompt = build_lane_prompt(LanePromptInput {
            lane_id,
            mode: manifest
                .lane_modes
                .get(lane_id)
                .cloned()
                .unwrap_or(LaneMode::ReadOnly),
            task: &task,
            deferred_files: &manifest.deferred_files,
            writable_scope: manifest.writable_scopes.get(lane_id).map(String::as_str),
        });
        std::fs::write(prompts_dir.join(format!("{lane_id}.prompt.txt")), prompt).map_err(
            |err| AppError::new(1, format!("failed to write prompt for {lane_id}: {err}")),
        )?;
    }

    manifest
        .save(&manifest_path)
        .map_err(|err| AppError::new(1, format!("failed to write manifest: {err}")))?;

    let panes = tmux_backend::create_detached_session(&session_name, &cwd, lane_ids.len())
        .map_err(|err| AppError::new(1, format!("failed to create tmux session: {err}")))?;

    for (lane_id, pane_id) in lane_ids.iter().zip(panes.iter()) {
        let prompt_file = prompts_dir.join(format!("{lane_id}.prompt.txt"));
        let output_last_message = outputs_dir.join(format!("{lane_id}.last-message.txt"));
        let raw_stdout = outputs_dir.join(format!("{lane_id}.stdout.txt"));
        let done_file = outputs_dir.join(format!("{lane_id}.done"));
        let script_path = inputs_dir.join(format!("{lane_id}.sh"));
        let command = match manifest.lane_modes.get(lane_id) {
            Some(LaneMode::Writable) => build_writable_script(
                &cwd,
                prompt_file.as_path(),
                output_last_message.as_path(),
                raw_stdout.as_path(),
                done_file.as_path(),
                manifest.writable_scopes.get(lane_id).map(String::as_str),
            ),
            _ => build_read_only_script(
                &cwd,
                prompt_file.as_path(),
                output_last_message.as_path(),
                raw_stdout.as_path(),
                done_file.as_path(),
            ),
        };
        std::fs::write(&script_path, command).map_err(|err| {
            AppError::new(
                1,
                format!("failed to write lane script for {lane_id}: {err}"),
            )
        })?;
        tmux_backend::send_keys(
            pane_id,
            &format!("zsh {}", shell_escape(script_path.as_path())),
        )
        .map_err(|err| AppError::new(1, format!("failed to start lane {lane_id}: {err}")))?;
    }

    let timeout = Duration::from_secs(60);
    let has_writable = manifest
        .lane_modes
        .values()
        .any(|mode| *mode == LaneMode::Writable);
    wait_for_read_only_lanes(&lane_ids, &outputs_dir, &manifest.lane_modes, timeout);

    for (lane_id, pane_id) in lane_ids.iter().zip(panes.iter()) {
        let capture_path = captures_dir.join(format!("{lane_id}.pane.txt"));
        let capture = tmux_backend::capture_pane(pane_id)
            .unwrap_or_else(|err| format!("capture failed: {err}"));
        std::fs::write(&capture_path, capture).map_err(|err| {
            AppError::new(1, format!("failed to write capture for {lane_id}: {err}"))
        })?;
        manifest
            .capture_files
            .insert(lane_id.clone(), capture_path.to_string_lossy().to_string());
    }

    let mut parsed_results = Vec::new();
    let mut parse_failed = false;
    for lane_id in &lane_ids {
        if manifest.lane_modes.get(lane_id) == Some(&LaneMode::Writable) {
            continue;
        }
        let output_path = outputs_dir.join(format!("{lane_id}.last-message.txt"));
        match std::fs::read_to_string(&output_path) {
            Ok(raw) => match parse_lane_result(&raw) {
                Ok(parsed) => parsed_results.push(parsed),
                Err(err) => {
                    parse_failed = true;
                    manifest
                        .notes
                        .push(format!("{lane_id}: parse failed: {err}"));
                }
            },
            Err(err) => {
                parse_failed = true;
                manifest
                    .notes
                    .push(format!("{lane_id}: missing output: {err}"));
            }
        }
    }

    manifest.parse_status = if parse_failed {
        ParseStatus::Failed
    } else {
        ParseStatus::Success
    };

    let mut seen_files = BTreeSet::new();
    let mut conflict = false;
    for result in &parsed_results {
        for file in &result.proposed_files {
            if !seen_files.insert(file.clone()) {
                conflict = true;
                manifest
                    .notes
                    .push(format!("conflict detected for proposed file: {file}"));
            }
        }
    }
    manifest.conflict_status = if conflict {
        ConflictStatus::Detected
    } else {
        ConflictStatus::Clear
    };

    if has_writable {
        manifest.synthesis_status = SynthesisStatus::Blocked;
        manifest.notes.push(
            "synthesis blocked because writable interactive lanes require human completion".into(),
        );
    } else if parse_failed || conflict {
        manifest.synthesis_status = SynthesisStatus::Blocked;
    } else {
        let parsed_json = serde_json::to_string_pretty(&parsed_results).map_err(|err| {
            AppError::new(1, format!("failed to serialize parsed results: {err}"))
        })?;
        let manifest_json = manifest.to_json().map_err(|err| {
            AppError::new(
                1,
                format!("failed to serialize manifest for synthesis: {err}"),
            )
        })?;
        let synthesis_prompt = build_synthesis_prompt(&task, &parsed_json, &manifest_json);
        std::fs::write(prompts_dir.join("synthesis.prompt.txt"), synthesis_prompt)
            .map_err(|err| AppError::new(1, format!("failed to write synthesis prompt: {err}")))?;
        let synthesis = synthesize_results(&parsed_results, &manifest, &task);
        std::fs::write(outputs_dir.join("synthesis.md"), synthesis)
            .map_err(|err| AppError::new(1, format!("failed to write synthesis output: {err}")))?;
        manifest.synthesis_status = SynthesisStatus::Completed;
    }

    manifest
        .save(&manifest_path)
        .map_err(|err| AppError::new(1, format!("failed to persist manifest: {err}")))?;

    println!("run: {}", manifest.run_slug);
    println!("session: {session_name}");
    println!("manifest: {}", manifest_path.display());

    if manifest.synthesis_status == SynthesisStatus::Completed {
        Ok(())
    } else if manifest.conflict_status == ConflictStatus::Detected || has_writable || parse_failed {
        Err(AppError::new(
            2,
            format!("run {} requires review", manifest.run_slug),
        ))
    } else {
        Err(AppError::new(
            1,
            format!("run {} failed", manifest.run_slug),
        ))
    }
}

fn inspect_command(args: InspectCommand) -> Result<(), AppError> {
    let run_root = std::env::current_dir()
        .map_err(|err| AppError::new(1, format!("failed to read cwd: {err}")))?
        .join("tmp")
        .join("executainer")
        .join(&args.run_slug);
    let manifest_path = run_root.join("manifest.json");
    let manifest = RunManifest::load(&manifest_path).map_err(|_| {
        AppError::new(
            1,
            format!(
                "run {} does not exist at {}",
                args.run_slug,
                manifest_path.display()
            ),
        )
    })?;

    if args.json {
        print!(
            "{}",
            manifest
                .to_json()
                .map_err(|err| AppError::new(1, format!("failed to serialize manifest: {err}")))?
        );
    } else {
        println!("run: {}", manifest.run_slug);
        println!("created: {}", manifest.created_at);
        println!("cwd: {}", manifest.cwd.display());
        println!("lanes: {}", manifest.requested_lane_count);
        println!("parse: {:?}", manifest.parse_status);
        println!("synthesis: {:?}", manifest.synthesis_status);
        println!("conflict: {:?}", manifest.conflict_status);
        if let Some(session_name) = manifest.session_name {
            println!("session: {session_name}");
        }
    }
    Ok(())
}

fn clean_command(args: CleanCommand) -> Result<(), AppError> {
    if !args.yes {
        return Err(AppError::new(1, "clean requires --yes"));
    }

    let run_root = std::env::current_dir()
        .map_err(|err| AppError::new(1, format!("failed to read cwd: {err}")))?
        .join("tmp")
        .join("executainer")
        .join(&args.run_slug);
    let manifest_path = run_root.join("manifest.json");
    if let Ok(manifest) = RunManifest::load(&manifest_path) {
        if let Some(session_name) = manifest.session_name {
            if tmux_backend::session_exists(&session_name) {
                let _ = tmux_backend::kill_session(&session_name);
            }
        }
    }

    std::fs::remove_dir_all(&run_root).map_err(|err| {
        AppError::new(
            1,
            format!(
                "failed to remove run {} at {}: {err}",
                args.run_slug,
                run_root.display()
            ),
        )
    })?;
    println!("removed {}", run_root.display());
    Ok(())
}

fn resolve_task(task: Option<String>, task_file: Option<PathBuf>) -> Result<String, AppError> {
    match (task, task_file) {
        (Some(task), None) => Ok(task),
        (None, Some(path)) => std::fs::read_to_string(&path).map_err(|err| {
            AppError::new(
                1,
                format!("failed to read task file {}: {err}", path.display()),
            )
        }),
        (Some(_), Some(_)) => Err(AppError::new(1, "choose either --task or --task-file")),
        (None, None) => Err(AppError::new(1, "run requires --task or --task-file")),
    }
}

fn build_run_slug() -> String {
    let stamp = Utc::now().format("%Y%m%d-%H%M%S").to_string();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.subsec_nanos())
        .unwrap_or_default();
    let suffix = format!("{:06x}", nanos & 0x00ff_ffff);
    format!("{stamp}-{suffix}")
}

fn create_run_dirs(run_root: &Path) -> Result<(), AppError> {
    for dir in ["inputs", "prompts", "captures", "outputs"] {
        std::fs::create_dir_all(run_root.join(dir)).map_err(|err| {
            AppError::new(
                1,
                format!("failed to create run dir {}: {err}", run_root.display()),
            )
        })?;
    }
    Ok(())
}

fn parse_writable_lanes(
    entries: &[String],
    lane_count: usize,
) -> Result<BTreeMap<String, String>, AppError> {
    let mut scopes = BTreeMap::new();
    for entry in entries {
        let (lane_id, scope) = entry
            .split_once(':')
            .ok_or_else(|| AppError::new(1, format!("invalid --writable-lane value: {entry}")))?;
        if !lane_id.starts_with("lane-") {
            return Err(AppError::new(
                1,
                format!("invalid lane id in --writable-lane: {lane_id}"),
            ));
        }
        let numeric = lane_id
            .strip_prefix("lane-")
            .and_then(|value| value.parse::<usize>().ok())
            .ok_or_else(|| {
                AppError::new(1, format!("invalid lane id in --writable-lane: {lane_id}"))
            })?;
        if numeric == 0 || numeric > lane_count {
            return Err(AppError::new(
                1,
                format!("writable lane {lane_id} exceeds lane count {lane_count}"),
            ));
        }
        scopes.insert(lane_id.to_string(), scope.to_string());
    }
    Ok(scopes)
}

fn build_read_only_script(
    cwd: &Path,
    prompt_file: &Path,
    output_last_message: &Path,
    raw_stdout: &Path,
    done_file: &Path,
) -> String {
    format!(
        r#"#!/bin/zsh
set +e
cat {prompt_file} | codex exec - --skip-git-repo-check -C {cwd} -s read-only -o {output_last_message} > {raw_stdout} 2>&1
status=$?
echo "$status" > {done_file}
exit 0
"#,
        prompt_file = shell_escape(prompt_file),
        cwd = shell_escape(cwd),
        output_last_message = shell_escape(output_last_message),
        raw_stdout = shell_escape(raw_stdout),
        done_file = shell_escape(done_file),
    )
}

fn build_writable_script(
    cwd: &Path,
    prompt_file: &Path,
    output_last_message: &Path,
    raw_stdout: &Path,
    done_file: &Path,
    writable_scope: Option<&str>,
) -> String {
    let scope = writable_scope.unwrap_or(".");
    format!(
        r#"#!/bin/zsh
echo "[executainer] writable lane scope: {scope}" > {raw_stdout}
echo "[executainer] interactive lane started" >> {raw_stdout}
echo "pending" > {done_file}
codex "$(cat {prompt_file})" -C {cwd} -s workspace-write -a on-request --no-alt-screen >> {raw_stdout} 2>&1
status=$?
echo "$status" > {done_file}
if [ -f {output_last_message} ]; then
  true
else
  echo "writable lane requires manual inspection" > {output_last_message}
fi
exit 0
"#,
        scope = scope,
        prompt_file = shell_escape(prompt_file),
        cwd = shell_escape(cwd),
        output_last_message = shell_escape(output_last_message),
        raw_stdout = shell_escape(raw_stdout),
        done_file = shell_escape(done_file),
    )
}

fn wait_for_read_only_lanes(
    lane_ids: &[String],
    outputs_dir: &Path,
    lane_modes: &BTreeMap<String, LaneMode>,
    timeout: Duration,
) {
    let start = std::time::Instant::now();
    loop {
        let all_done = lane_ids
            .iter()
            .filter(|lane_id| lane_modes.get(*lane_id) != Some(&LaneMode::Writable))
            .all(|lane_id| outputs_dir.join(format!("{lane_id}.done")).exists());
        if all_done || start.elapsed() > timeout {
            break;
        }
        thread::sleep(Duration::from_millis(300));
    }
}

fn shell_escape(path: &Path) -> String {
    let value = path.to_string_lossy();
    format!("'{}'", value.replace('\'', r"'\''"))
}
