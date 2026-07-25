use kani_domain_api_model::import::{ImportSession, ImportSessionId};
use kani_domain_api_model::user::User;

#[cfg_attr(feature = "test-util", mockall::automock)]
pub trait ImportSessionDatabase {
    fn get_import_sessions(&self, user: &User) -> Result<Vec<ImportSession>, anyhow::Error>;

    fn get_import_session_by_id_and_user(&self, user: &User, import_session_id: ImportSessionId) -> Result<Option<ImportSession>, anyhow::Error>;
}
