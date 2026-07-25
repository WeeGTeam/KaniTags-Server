use crate::converter::FromDomain;
use kani_domain_api_model::import::ImportSession;
use kani_openapi::models::ImportSessionDto;

impl FromDomain<ImportSession> for ImportSessionDto {
    fn from_domain(session: ImportSession) -> Self {
        Self {
            id: session.id.to_string(),
            created_at: session.created_at,
            updated_at: session.updated_at,
            closed_at: session.closed_at,
        }
    }
}
