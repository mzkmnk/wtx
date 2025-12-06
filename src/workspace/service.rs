use crate::{
    config::manager::ConfigManager,
    models::WtxError,
    workspace::{file::WorkspaceFileManager, worktree::WorktreeManager},
};

/// en: Service for generating workspaces with worktrees
///
/// ja: worktreeを含むworkspaceを生成するサービス
pub struct WorkspaceGenerationService<W: WorktreeManager> {
    config_manager: ConfigManager,
    worktree_manager: W,
    workspace_file_manager: WorkspaceFileManager,
}

impl<W: WorktreeManager> WorkspaceGenerationService<W> {
    pub fn new(worktree_manager: W) -> Result<Self, WtxError> {
        Ok(Self {
            config_manager: ConfigManager::new()?,
            workspace_file_manager: WorkspaceFileManager::default(),
            worktree_manager,
        })
    }

    pub fn get_branches(&self, repo_name: &str) -> Result<Vec<String>, WtxError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::workspace::worktree::MockWorktreeManager;

    use super::*;

    #[test]
    fn test_new() {
        let mock_worktree_manager = MockWorktreeManager::new();

        let workspace_generation_service = WorkspaceGenerationService::new(mock_worktree_manager);
        assert!(workspace_generation_service.is_ok());
    }
}
