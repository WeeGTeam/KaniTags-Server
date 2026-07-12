use crate::dao::Dao;
use crate::Postgres;
use diesel::Connection;
use kani_domain_api_model::import::{ImportSession, ImportSessionId};
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

    fn get_import_session_by_id_and_user(&self, user: &User, import_session_id: ImportSessionId) -> Result<Option<ImportSession>, anyhow::Error> {
        debug!("Retrieving import session for user '{}' and session id '{}'", user.user_name, *import_session_id);
        let mut connection = self.get_connection()?;
        let row = connection.transaction(|conn| {
            conn.import_session_dao()
                .get_import_session_by_id_and_user(*import_session_id, user.id)
        })?;
        debug!("Retrieved import session for user '{}' and session id '{}': {}", user.user_name, *import_session_id, row.is_some());
        Ok(row.map(|it| it.into()))
    }

}
