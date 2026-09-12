use kani_domain_api_model::user::User;

#[cfg_attr(feature = "test-util", mockall::automock)]
#[async_trait::async_trait]
pub trait UserDatabase {
    async fn get_user_by_user_name(&self, user_name: String) -> Result<Option<User>, anyhow::Error>;
}
