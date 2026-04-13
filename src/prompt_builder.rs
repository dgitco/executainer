use crate::manifest::LaneMode;
use crate::result_parser::{SENTINEL_END, SENTINEL_START};

const READ_ONLY_TEMPLATE: &str = include_str!("../templates/lane_read_only.txt");
const WRITABLE_TEMPLATE: &str = include_str!("../templates/lane_writable.txt");
const SYNTHESIS_TEMPLATE: &str = include_str!("../templates/synthesis.txt");

pub struct LanePromptInput<'a> {
    pub lane_id: &'a str,
    pub mode: LaneMode,
    pub task: &'a str,
    pub deferred_files: &'a [String],
    pub writable_scope: Option<&'a str>,
}

pub fn build_lane_prompt(input: LanePromptInput<'_>) -> String {
    let template = match input.mode {
        LaneMode::ReadOnly => READ_ONLY_TEMPLATE,
        LaneMode::Writable => WRITABLE_TEMPLATE,
    };
    let deferred_files = if input.deferred_files.is_empty() {
        "- none".to_string()
    } else {
        input
            .deferred_files
            .iter()
            .map(|file| format!("- {file}"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let writable_scope = input.writable_scope.unwrap_or("none");
    template
        .replace("{{LANE_ID}}", input.lane_id)
        .replace("{{TASK}}", input.task)
        .replace("{{DEFERRED_FILES}}", &deferred_files)
        .replace("{{WRITABLE_SCOPE}}", writable_scope)
        .replace("{{SENTINEL_START}}", SENTINEL_START)
        .replace("{{SENTINEL_END}}", SENTINEL_END)
}

pub fn build_synthesis_prompt(
    task: &str,
    parsed_results_json: &str,
    manifest_json: &str,
) -> String {
    SYNTHESIS_TEMPLATE
        .replace("{{TASK}}", task)
        .replace("{{PARSED_RESULTS_JSON}}", parsed_results_json)
        .replace("{{MANIFEST_JSON}}", manifest_json)
}
