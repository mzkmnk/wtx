use crate::{
    models::{RegistrationError, Repository},
    service::repository::RepositoryService,
};

pub fn execute() -> Result<Vec<Repository>, RegistrationError> {
    let repository_service = RepositoryService::new()?;
    Ok(repository_service.list()?)
}
