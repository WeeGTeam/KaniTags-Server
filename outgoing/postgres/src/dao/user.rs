use crate::models::user_account::{UserAccountInsertRow, UserAccountRow};
use crate::schema::user_account::dsl as user_dsl;
use crate::schema::user_account::dsl::user_account;
use anyhow::{Context, Error};
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

pub struct UserDao<'c> {
    connection: &'c mut diesel::PgConnection,
}

impl<'c> UserDao<'c> {
    pub fn new(connection: &'c mut diesel::PgConnection) -> Self {
        UserDao { connection }
    }

    pub fn insert_user(
        &mut self,
        insert_row: &UserAccountInsertRow,
    ) -> Result<UserAccountRow, Error> {
        diesel::insert_into(user_account)
            .values(insert_row)
            .returning(UserAccountRow::as_returning())
            .get_result(self.connection)
            .context("Failed to insert user into database.")
    }

    pub fn get_all_users(&mut self) -> Result<Vec<UserAccountRow>, Error> {
        user_account
            .load(self.connection)
            .context("Failed to load all users from database.")
    }

    pub fn get_user_by_user_name(&mut self, user_name: &str) -> Result<UserAccountRow, Error> {
        user_account
            .select(UserAccountRow::as_select())
            .filter(user_dsl::user_name.eq(user_name))
            .get_result(self.connection)
            .context("Failed to load user by user name from database.")
    }
}

#[cfg(test)]
mod test {
    use crate::dao::test::insert_test_user;
    use crate::dao::Dao;
    use crate::models::user_account::UserAccountInsertRow;
    use crate::test::test_db;
    use diesel::Connection;

    #[test]
    #[serial_test::serial]
    fn test_insert_user() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            c.user_dao().insert_user(&UserAccountInsertRow {
                user_name: "test_user".to_string(),
                display_name: "Test User".to_string(),
            })
        });
        println!("user: {:?}", result);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_all_users() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let _user = insert_test_user(c)?;
            c.user_dao().get_all_users()
        });
        for result in results {
            println!("user: {:?}", result);
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_get_user_by_user_name() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            c.user_dao().get_user_by_user_name(&user.user_name)
        });
        println!("user: {:?}", result);
    }
}
