use kani_domain_api_model::user::User;

pub trait UserDatabase {
    fn get_user_by_user_name(&self, user_name: &str) -> Result<Option<User>, anyhow::Error>;
}


#[cfg(feature = "test-util")]
pub mod mock {
    use super::*;

    pub struct MockUserDatabase {
        pub get_user_by_user_name_fn: Box<dyn Fn(&str) -> Result<Option<User>, anyhow::Error> + Send + Sync>,
    }

    impl Default for MockUserDatabase {
        fn default() -> Self {
            Self {
                get_user_by_user_name_fn: Box::new(|_| unimplemented!("get_user_by_user_name was not configured")),
            }
        }
    }

    impl MockUserDatabase {
        pub fn with_get_user_by_user_name(
            mut self,
            f: impl Fn(&str) -> Result<Option<User>, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.get_user_by_user_name_fn = Box::new(f);
            self
        }
    }

    impl UserDatabase for MockUserDatabase {
        fn get_user_by_user_name(&self, user_name: &str) -> Result<Option<User>, anyhow::Error> {
            (self.get_user_by_user_name_fn)(user_name)
        }
    }
}
