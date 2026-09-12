use crate::Postgres;
use crate::dao::Dao;
use anyhow::Error;
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::error::ReadDbError;
use kani_domain_api_outgoing::database::user_database::UserDatabase;

#[async_trait::async_trait]
impl UserDatabase for Postgres {
    async fn get_user_by_user_name(&self, user_name: String) -> Result<Option<User>, Error> {
        let user_row = self.transaction::<_, _, ReadDbError>(move |conn| {
            conn.user_dao().get_user_by_user_name(&user_name)
        }).await?;
        Ok(user_row.map(Into::into))
    }
}
