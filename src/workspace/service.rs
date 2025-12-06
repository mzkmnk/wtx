use crate::{
    config::manager::ConfigManager,
    models::WtxError,
    workspace::{file::WorkspaceFileManager, worktree::WorktreeManager},
};

/// en: Service for generating workspaces with worktrees
///
/// ja: worktreeを含むworkspaceを生成するサービス
pub struct WorkspaceGenerationService {
    config_manager: ConfigManager,
    worktree_manager: WorktreeManager,
    workspace_file_manager: WorkspaceFileManager,
}

impl WorkspaceGenerationService {
    pub fn new() -> Result<Self, WtxError> {
        Ok(Self {
            config_manager: ConfigManager::new()?,
            worktree_manager: WorktreeManager::default(),
            workspace_file_manager: WorkspaceFileManager::default(),
        })
    }
}
