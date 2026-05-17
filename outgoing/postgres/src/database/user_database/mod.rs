use crate::dao::Dao;
use crate::models::user_account::UserAccountRow;
use crate::Postgres;
use anyhow::Error;
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::user_database::UserDatabase;

impl UserDatabase for Postgres {
    fn get_user_by_user_name(&self, user_name: &str) -> Result<Option<User>, Error> {
        let mut connection = self.get_connection()?;
        let user_row = connection.user_dao().get_user_by_user_name(user_name)?;
        Ok(user_row.map(Into::into))
    }
}

impl Into<User> for UserAccountRow {
    fn into(self) -> User {
        User {
            id: self.id,
            user_name: self.user_name,
            display_name: self.display_name,
        }
    }
}
