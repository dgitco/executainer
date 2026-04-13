use std::path::Path;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TmuxError {
    #[error("tmux command failed: {0}")]
    CommandFailed(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

fn run_tmux(args: &[&str]) -> Result<String, TmuxError> {
    let output = Command::new("tmux").args(args).output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(TmuxError::CommandFailed(stderr));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn create_detached_session(
    session_name: &str,
    cwd: &Path,
    pane_count: usize,
) -> Result<Vec<String>, TmuxError> {
    let cwd = cwd.to_string_lossy();
    let first_pane = run_tmux(&[
        "new-session",
        "-d",
        "-P",
        "-F",
        "#{pane_id}",
        "-s",
        session_name,
        "-c",
        &cwd,
    ])?;
    let mut panes = vec![first_pane];

    for _ in 1..pane_count {
        let pane = run_tmux(&[
            "split-window",
            "-t",
            session_name,
            "-P",
            "-F",
            "#{pane_id}",
            "-c",
            &cwd,
        ])?;
        panes.push(pane);
    }

    let _ = run_tmux(&["select-layout", "-t", session_name, "tiled"])?;
    Ok(panes)
}

pub fn send_keys(target: &str, command: &str) -> Result<(), TmuxError> {
    let _ = run_tmux(&["send-keys", "-t", target, command, "C-m"])?;
    Ok(())
}

pub fn capture_pane(target: &str) -> Result<String, TmuxError> {
    run_tmux(&["capture-pane", "-p", "-t", target])
}

pub fn kill_session(session_name: &str) -> Result<(), TmuxError> {
    let _ = run_tmux(&["kill-session", "-t", session_name])?;
    Ok(())
}

pub fn session_exists(session_name: &str) -> bool {
    Command::new("tmux")
        .args(["has-session", "-t", session_name])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}
