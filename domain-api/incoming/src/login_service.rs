use async_trait::async_trait;
use kani_domain_api_model::user::User;
use thiserror::Error;

#[async_trait]
pub trait LoginService {
    async fn load_user_by_user_name(&self, user_name: &str) -> Result<User, UserLoadError>;
}

#[derive(Error, Debug)]
pub enum UserLoadError {
    #[error("User load internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),

    #[error("Unable to find user: '{0}'")]
    UserMissingError(String),
}
