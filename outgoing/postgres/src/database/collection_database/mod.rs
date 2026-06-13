use crate::Postgres;
use anyhow::Error;
use kani_domain_api_model::collection::{Collection, CollectionId};
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::collection_database::CollectionDatabase;

impl CollectionDatabase for Postgres {
    fn load_collection_by_user_and_id(&self, user: &User, collection_id: CollectionId) -> Result<Option<Collection>, Error> {
        todo!()
    }

    fn load_collection_by_user_and_name(&self, user: &User, collection_name: &str) -> Result<Option<Collection>, Error> {
        todo!()
    }

    fn load_collections_by_user(&self, user: &User) -> Result<Vec<Collection>, Error> {
        todo!()
    }

    fn create_collection(&self, user: &User, name: &str) -> Result<Collection, Error> {
        todo!()
    }

    fn delete_collection(&self, user: &User, collection_id: CollectionId) -> Result<(), Error> {
        todo!()
    }

    fn add_images_to_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<usize, Error> {
        todo!()
    }

    fn remove_images_from_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<usize, Error> {
        todo!()
    }
}
