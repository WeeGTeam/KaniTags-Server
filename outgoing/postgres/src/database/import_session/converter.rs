use crate::models::import_session::ImportSessionRow;
use kani_domain_api_model::import::{ImportSession, ImportSessionId};

impl Into<ImportSessionId> for ImportSessionRow {
    fn into(self) -> ImportSessionId {
        ImportSessionId(self.id)
    }
}

impl Into<ImportSession> for ImportSessionRow {
    fn into(self) -> ImportSession {
        ImportSession {
            id: self.id,
            user_id: self.user_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            closed_at: self.closed_at,
        }
    }
}
