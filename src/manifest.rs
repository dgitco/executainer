use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LaneMode {
    ReadOnly,
    Writable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ParseStatus {
    Pending,
    Success,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SynthesisStatus {
    Pending,
    Completed,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConflictStatus {
    Clear,
    Detected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApprovalEvent {
    pub lane_id: String,
    pub command: String,
    pub recommendation: String,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunManifest {
    pub run_slug: String,
    pub created_at: String,
    pub cwd: PathBuf,
    pub requested_lane_count: usize,
    pub lane_ids: Vec<String>,
    pub lane_modes: BTreeMap<String, LaneMode>,
    pub writable_scopes: BTreeMap<String, String>,
    pub prompt_template_refs: BTreeMap<String, String>,
    pub deferred_files: Vec<String>,
    pub capture_files: BTreeMap<String, String>,
    pub parse_status: ParseStatus,
    pub synthesis_status: SynthesisStatus,
    pub conflict_status: ConflictStatus,
    pub session_name: Option<String>,
    pub approval_events: Vec<ApprovalEvent>,
    pub notes: Vec<String>,
}

impl RunManifest {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        let mut json = serde_json::to_string_pretty(self)?;
        json.push('\n');
        Ok(json)
    }

    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        std::fs::write(path, self.to_json().map_err(std::io::Error::other)?)
    }

    pub fn load(path: &Path) -> Result<Self, std::io::Error> {
        let raw = std::fs::read_to_string(path)?;
        serde_json::from_str(&raw).map_err(std::io::Error::other)
    }
}
