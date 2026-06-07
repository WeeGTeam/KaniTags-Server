pub mod converter;

use crate::dao::Dao;
use crate::Postgres;
use diesel::Connection;
use kani_domain_api_model::import::ImportSession;
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::import_session::ImportSessionDatabase;
use tracing::debug;

impl ImportSessionDatabase for Postgres {
    fn get_import_sessions(&self, user: &User) -> Result<Vec<ImportSession>, anyhow::Error> {
        debug!("Getting import sessions by user: {}", user.user_name);
        let mut connection = self.get_connection()?;
        let rows = connection.transaction(|conn| {{
            conn.import_session_dao().get_all_import_sessions_of_user(user.id)
        }})?;
        debug!("Finished getting import sessions with {} results", rows.len());
        Ok(rows.into_iter().map(Into::into).collect())
    }
}
