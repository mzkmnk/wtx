use std::path::Path;

pub struct WorktreeManager;

impl WorktreeManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_worktree(&self, bare_repo_path: &Path, target_path: &Path, branch: &str) {
        todo!()
    }

    pub fn remove_worktree(&self) {
        todo!()
    }

    pub fn list_worktrees(&self) {
        todo!()
    }

    pub fn fetch(&self) {
        todo!()
    }

    pub fn get_remote_branches(&self) {
        todo!()
    }

    pub fn branch_exists(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_worktree_manager_create_worktree() {
        todo!()
    }
}
