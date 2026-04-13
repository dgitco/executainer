use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const SENTINEL_START: &str = "<<<EXECUTAINER_RESULT_START>>>";
pub const SENTINEL_END: &str = "<<<EXECUTAINER_RESULT_END>>>";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParsedLaneResult {
    pub lane_id: String,
    pub status: String,
    pub summary: String,
    pub proposed_files: Vec<String>,
    pub deferred_files: Vec<String>,
    pub notes: String,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("sentinel block missing or duplicated")]
    MissingSentinel,
    #[error("sentinel payload must be valid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
}

pub fn parse_lane_result(raw: &str) -> Result<ParsedLaneResult, ParseError> {
    let start_positions = raw.match_indices(SENTINEL_START).collect::<Vec<_>>();
    let end_positions = raw.match_indices(SENTINEL_END).collect::<Vec<_>>();
    if start_positions.len() != 1 || end_positions.len() != 1 {
        return Err(ParseError::MissingSentinel);
    }

    let start_idx = start_positions[0].0 + SENTINEL_START.len();
    let end_idx = end_positions[0].0;
    if end_idx <= start_idx {
        return Err(ParseError::MissingSentinel);
    }

    let payload = raw[start_idx..end_idx].trim();
    serde_json::from_str(payload).map_err(ParseError::InvalidJson)
}
