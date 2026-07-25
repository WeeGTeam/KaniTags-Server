use crate::database::error::{DbError, ReadDbError, ReadWriteDbError, WriteDbError};
use kani_domain_api_model::collection::{Collection, CollectionId, CollectionName};
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::user::User;

#[cfg_attr(feature = "test-util", mockall::automock)]
#[async_trait::async_trait]
pub trait CollectionDatabase {
    async fn load_collection_by_user_and_id(&self, user: &User, collection_id: CollectionId) -> Result<Collection, ReadDbError>;
    async fn load_collection_by_user_and_name(&self, user: &User, collection_name: &CollectionName) -> Result<Collection, ReadDbError>;
    async fn load_collections_by_user(&self, user: &User) -> Result<Vec<Collection>, DbError>;
    async fn create_collection(&self, user: &User, name: &CollectionName) -> Result<Collection, WriteDbError>;
    async fn delete_collection(&self, user: &User, collection_id: CollectionId) -> Result<(), ReadDbError>;
    async fn add_images_to_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<usize, ReadWriteDbError>;
    async fn remove_images_from_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<usize, DbError>;
}
