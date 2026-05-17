use async_trait::async_trait;
use kani_domain_api_incoming::login_service::{LoginService, UserLoadError};
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::Database;
use std::sync::Arc;

pub struct LoginServiceImpl {
    database: Arc<dyn Database + Sync + Send>,
}

impl LoginServiceImpl {
    pub fn new(database: Arc<dyn Database + Send + Sync>) -> Self {
        Self { database }
    }
}

#[async_trait]
impl LoginService for LoginServiceImpl {
    async fn load_user_by_user_name(&self, user_name: &str) -> Result<User, UserLoadError> {
        match self.database.get_user_by_user_name(user_name)? {
            Some(user) => Ok(user),
            None => Err(UserLoadError::UserMissingError(user_name.to_owned())),
        }
    }
}
