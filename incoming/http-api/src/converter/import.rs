use crate::converter::FromDomain;
use kani_openapi::models::ImportSession;

impl FromDomain<kani_domain_api_model::import::ImportSession> for ImportSession {
    fn from_domain(session: kani_domain_api_model::import::ImportSession) -> Self {
        Self {
            id: session.id.to_string(),
            created_at: session.created_at,
            updated_at: session.updated_at,
            closed_at: session.closed_at,
        }
    }
}
