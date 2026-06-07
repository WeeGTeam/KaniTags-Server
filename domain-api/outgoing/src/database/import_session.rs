use kani_domain_api_model::import::ImportSession;
use kani_domain_api_model::user::User;

pub trait ImportSessionDatabase {
    fn get_import_sessions(&self, user: &User) -> Result<Vec<ImportSession>, anyhow::Error>;
}
