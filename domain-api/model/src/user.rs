#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: i64,
    pub user_name: String,
    pub display_name: String,
}


#[cfg(feature = "test-util")]
pub mod stub {
    use super::*;

    impl User {
        pub fn stub() -> Self {
            User {
                id: i64::MAX,
                user_name: "stub_user_name".to_owned(),
                display_name: "stub_display_name".to_owned(),
            }
        }
    }
}
