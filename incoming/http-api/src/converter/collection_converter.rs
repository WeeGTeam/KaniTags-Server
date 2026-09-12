use crate::converter::FromDomain;
use kani_domain_api_model::collection::Collection;
use kani_openapi::models::CollectionDto;

impl FromDomain<Collection> for CollectionDto {
    fn from_domain(collection: Collection) -> Self {
        CollectionDto {
            id: collection.id.to_string(),
            created_at: collection.created_at,
            created_by: collection.created_by.to_string(),
            updated_at: collection.updated_at,
            name: collection.name.to_string()
        }
    }
}
