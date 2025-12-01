use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct WorktreeSelection {
    pub repo_name: String,
    pub branch: String,
}

#[derive(Debug, Clone)]
pub struct WorktreeInfo {
    pub path: PathBuf,
    pub branch: String,
    pub repo_name: String,
}

#[derive(Debug, Clone)]
pub struct BranchInfo {
    pub name: String,
    pub is_default: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceSettings {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFile {
    pub folders: Vec<WorkspaceFolder>,
    pub settings: WorkspaceSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFolder {
    pub path: String,
}

impl WorkspaceFile {
    pub fn new(folders: Vec<String>) -> Self {
        Self {
            folders: folders
                .into_iter()
                .map(|f| WorkspaceFolder { path: f })
                .collect(),
            settings: WorkspaceSettings::default(),
        }
    }
}
