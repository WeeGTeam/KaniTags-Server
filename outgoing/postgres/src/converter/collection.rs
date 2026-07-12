use crate::models::collection::CollectionRow;
use kani_domain_api_model::collection::{Collection, CollectionId, CollectionName};

impl TryFrom<CollectionRow> for Collection {
    type Error = anyhow::Error;
    fn try_from(row: CollectionRow) -> Result<Self, Self::Error> {
        Ok(
            Collection {
                id: CollectionId(row.id),
                name: CollectionName::try_from(row.name)?,
                created_by: row.user_id,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }
        )
    }
}
