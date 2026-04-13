use crate::manifest::RunManifest;
use crate::result_parser::ParsedLaneResult;

pub fn synthesize_results(
    results: &[ParsedLaneResult],
    manifest: &RunManifest,
    task: &str,
) -> String {
    let summaries = results
        .iter()
        .map(|result| format!("- {}: {}", result.lane_id, result.summary))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "# Executainer synthesis\n\nTask: {task}\n\nRun: {}\n\nLane summaries:\n{}\n",
        manifest.run_slug, summaries
    )
}
