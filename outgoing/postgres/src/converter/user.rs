use crate::models::user_account::UserAccountRow;
use kani_domain_api_model::user::User;

impl Into<User> for UserAccountRow {
    fn into(self) -> User {
        User {
            id: self.id,
            user_name: self.user_name,
            display_name: self.display_name,
        }
    }
}
