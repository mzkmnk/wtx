use crate::{models::RegistrationError, service::repository::RepositoryService};

pub fn execute(name: &str) -> Result<(), RegistrationError> {
    let mut repository_service = RepositoryService::new()?;
    repository_service.unregister(name)?;
    Ok(())
}
