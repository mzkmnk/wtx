use crate::{models::RegistrationError, service::repository::RepositoryService};

pub fn execute(url: &str) -> Result<(), RegistrationError> {
    let mut repository_service = RepositoryService::new()?;
    repository_service.register(url)?;
    Ok(())
}
