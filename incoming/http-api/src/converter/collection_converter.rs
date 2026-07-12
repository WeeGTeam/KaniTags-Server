use crate::converter::FromDomain;
use kani_openapi::models::Collection;

impl FromDomain<kani_domain_api_model::collection::Collection> for Collection {
    fn from_domain(collection: kani_domain_api_model::collection::Collection) -> Self {
        Collection {
            id: collection.id.to_string(),
            created_at: collection.created_at,
            created_by: collection.created_by.to_string(),
            updated_at: collection.updated_at,
            name: collection.name
        }
    }
}
