use crate::models::collection::CollectionRow;
use kani_domain_api_model::collection::Collection;

impl From<CollectionRow> for Collection {
    fn from(row: CollectionRow) -> Self {
        Collection {
            id: row.id,
            name: row.name,
            created_by: row.user_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
