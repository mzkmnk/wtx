use std::path::Path;

use crate::models::RegistrationError;

pub struct GitOperations;

impl GitOperations {
    pub fn new() -> Self {
        Self {}
    }

    pub fn validate_url(&self, url: &str) -> Result<(), RegistrationError> {
        todo!()
    }

    pub fn extract_repo_name(&self, url: &str) -> Result<String, RegistrationError> {
        todo!()
    }

    pub fn bare_clone(&self, url: &str, target_path: &Path) -> Result<(), RegistrationError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use tempfile::{tempdir, TempDir};

    use super::*;

    fn create_temp_dir() -> TempDir {
        tempdir().unwrap()
    }

    #[test]
    fn test_validate_url_https_valid() {
        let git_operations = GitOperations::new();

        assert!(git_operations
            .validate_url("https://github.com/org/repo.git")
            .is_ok());

        assert!(git_operations
            .validate_url("https://github.com/org/repo")
            .is_ok());

        assert!(git_operations
            .validate_url("https://github.com/org/team/repo.git")
            .is_ok());
    }

    #[test]
    fn test_validate_url_ssh_valid() {
        let git_operations = GitOperations::new();

        assert!(git_operations
            .validate_url("git@github.com:org/repo.git")
            .is_ok());

        assert!(git_operations
            .validate_url("git@github.com:org/repo")
            .is_ok());

        assert!(git_operations
            .validate_url("git@github.com:org/team/repo.git")
            .is_ok());

        assert!(git_operations
            .validate_url("git@github.com:my_org/repo.git")
            .is_ok());
    }

    #[test]
    fn test_validate_url_invalid() {
        let git_operations = GitOperations::new();

        assert!(git_operations.validate_url("").is_err());

        assert!(git_operations.validate_url("https://github.com").is_err());

        assert!(git_operations
            .validate_url("git@github.com/org/repo.git")
            .is_err());
    }

    #[test]
    fn test_extract_repo_name() {
        let git_operations = GitOperations::new();

        assert_eq!(
            git_operations
                .extract_repo_name("https://github.com/org/repo.git")
                .unwrap(),
            "repo".to_string()
        );

        assert_eq!(
            git_operations
                .extract_repo_name("https://github.com/org/repo")
                .unwrap(),
            "repo".to_string()
        );

        assert_eq!(
            git_operations
                .extract_repo_name("https://github.com/org/team/repo.git")
                .unwrap(),
            "repo".to_string()
        );

        assert_eq!(
            git_operations
                .extract_repo_name("git@github.com:org/repo.git")
                .unwrap(),
            "repo".to_string()
        );

        assert_eq!(
            git_operations
                .extract_repo_name("git@github.com:org/team/repo.git")
                .unwrap(),
            "repo".to_string(),
        );

        assert_eq!(
            git_operations
                .extract_repo_name("git@github.com:org/repo")
                .unwrap(),
            "repo".to_string()
        );

        assert_eq!(
            git_operations
                .extract_repo_name("git@github.com:my_org/repo.git")
                .unwrap(),
            "repo".to_string()
        );
    }

    #[test]
    fn test_bare_clone() {
        let dir = create_temp_dir();

        let target_path = dir.path().join("target.git");
        let source_repo = dir.path().join("source");
        git2::Repository::init(&source_repo).unwrap();

        let git_operations = GitOperations::new();

        assert!(git_operations
            .bare_clone(source_repo.to_str().unwrap(), &target_path)
            .is_ok());
        assert!(git2::Repository::open_bare(&target_path).is_ok());
    }
}
