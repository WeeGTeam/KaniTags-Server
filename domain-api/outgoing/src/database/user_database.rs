use kani_domain_api_model::user::User;

pub trait UserDatabase {
    fn get_user_by_user_name(&self, user_name: &str) -> Result<Option<User>, anyhow::Error>;
}
