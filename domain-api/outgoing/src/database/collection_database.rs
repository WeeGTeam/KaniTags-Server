use kani_domain_api_model::collection::{Collection, CollectionId, CollectionName};
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::user::User;

#[cfg_attr(feature = "test-util", mockall::automock)]
pub trait CollectionDatabase {
    fn load_collection_by_user_and_id(&self, user: &User, collection_id: CollectionId) -> Result<Option<Collection>, anyhow::Error>;
    fn load_collection_by_user_and_name(&self, user: &User, collection_name: &CollectionName) -> Result<Option<Collection>, anyhow::Error>;
    fn load_collections_by_user(&self, user: &User) -> Result<Vec<Collection>, anyhow::Error>;
    fn create_collection(&self, user: &User, name: &CollectionName) -> Result<Collection, anyhow::Error>;
    fn delete_collection(&self, user: &User, collection_id: CollectionId) -> Result<(), anyhow::Error>;
    fn add_images_to_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<usize, anyhow::Error>;
    fn remove_images_from_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<usize, anyhow::Error>;
}
