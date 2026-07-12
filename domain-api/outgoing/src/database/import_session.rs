use kani_domain_api_model::import::{ImportSession, ImportSessionId};
use kani_domain_api_model::user::User;

pub trait ImportSessionDatabase {
    fn get_import_sessions(&self, user: &User) -> Result<Vec<ImportSession>, anyhow::Error>;

    fn get_import_session_by_id_and_user(&self, user: &User, import_session_id: ImportSessionId) -> Result<Option<ImportSession>, anyhow::Error>;
}


#[cfg(feature = "test-util")]
pub mod mock {
    use super::*;

    pub struct MockImportSessionDatabase {
        pub get_import_sessions_fn: Box<dyn Fn(&User) -> Result<Vec<ImportSession>, anyhow::Error> + Send + Sync>,
        pub get_import_session_by_id_and_user_fn: Box<dyn Fn(&User, ImportSessionId) -> Result<Option<ImportSession>, anyhow::Error> + Send + Sync>,
    }

    impl Default for MockImportSessionDatabase {
        fn default() -> Self {
            Self {
                get_import_sessions_fn: Box::new(|_| unimplemented!("get_import_sessions was not configured")),
                get_import_session_by_id_and_user_fn: Box::new(|_, _| unimplemented!("get_import_session_by_id_and_user was not configured")),
            }
        }
    }

    impl MockImportSessionDatabase {
        pub fn with_get_import_sessions(
            mut self,
            f: impl Fn(&User) -> Result<Vec<ImportSession>, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.get_import_sessions_fn = Box::new(f);
            self
        }

        pub fn with_get_import_session_by_id_and_user(
            mut self,
            f: impl Fn(&User, ImportSessionId) -> Result<Option<ImportSession>, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.get_import_session_by_id_and_user_fn = Box::new(f);
            self
        }
    }

    impl ImportSessionDatabase for MockImportSessionDatabase {
        fn get_import_sessions(&self, user: &User) -> Result<Vec<ImportSession>, anyhow::Error> {
            (self.get_import_sessions_fn)(user)
        }

        fn get_import_session_by_id_and_user(&self, user: &User, import_session_id: ImportSessionId) -> Result<Option<ImportSession>, anyhow::Error> {
            (self.get_import_session_by_id_and_user_fn)(user, import_session_id)
        }
    }
}
