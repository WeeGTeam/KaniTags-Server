use crate::dao::Dao;
use crate::models::collection::CollectionInsertRow;
use crate::models::collection_image::CollectionImageInsertRow;
use crate::Postgres;
use anyhow::Error;
use diesel::Connection;
use kani_domain_api_model::collection::{Collection, CollectionId};
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::collection_database::CollectionDatabase;
use tracing::debug;

impl CollectionDatabase for Postgres {
    fn load_collection_by_user_and_id(&self, user: &User, collection_id: CollectionId) -> Result<Option<Collection>, Error> {
        debug!("Loading collection by user and id: {}, {}", user.id, *collection_id);
        let mut connection = self.get_connection()?;
        let collection = connection.transaction(|conn| conn.collection_dao().get_collection_by_user_and_id(user.id, *collection_id))?;
        debug!("Loaded collection: {:?}", collection.as_ref().map(|c| c.id));
        Ok(collection.map(Into::into))
    }

    fn load_collection_by_user_and_name(&self, user: &User, collection_name: &str) -> Result<Option<Collection>, Error> {
        debug!("Loading collection by user and name: {}, {}", user.id, collection_name);
        let mut connection = self.get_connection()?;
        let collection = connection.transaction(|conn| conn.collection_dao().get_collection_by_user_and_name(user.id, collection_name))?;
        debug!("Loaded collection: {:?}", collection.as_ref().map(|c| c.id));
        Ok(collection.map(Into::into))
    }

    fn load_collections_by_user(&self, user: &User) -> Result<Vec<Collection>, Error> {
        debug!("Loading collections by user: {}", user.id);
        let mut connection = self.get_connection()?;
        let collections = connection.transaction(|conn| conn.collection_dao().get_collections_by_user(user.id))?;
        debug!("Loaded {:?} collections", collections.len());
        Ok(collections.into_iter().map(Into::into).collect())
    }

    fn create_collection(&self, user: &User, name: &str) -> Result<Collection, Error> {
        debug!("Creating collection for user: {}, name: {}", user.id, name);
        let mut connection = self.get_connection()?;
        let collection = connection.transaction(|conn| conn.collection_dao().insert_collection(
            &CollectionInsertRow {
                user_id: user.id,
                name: name.to_string()
            }
        ))?;
        debug!("Created collection: {:?}", collection.id);
        Ok(collection.into())
    }

    fn delete_collection(&self, user: &User, collection_id: CollectionId) -> Result<(), Error> {
        debug!("Deleting collection for user: {}, id: {}", user.id, *collection_id);
        let mut connection = self.get_connection()?;
        let collection = connection.transaction(|conn| conn.collection_dao().delete_collection(user.id, *collection_id))?;
        debug!("Deleted collection: {:?}", collection.id);
        Ok(())
    }

    fn add_images_to_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<usize, Error> {
        debug!("Adding {} images to collection for user: {}, collection_id: {}", image_ids.len(), user.id, *collection_id);
        let mut connection = self.get_connection()?;
        let inserted_images = connection.transaction(|conn| conn.collection_dao().insert_collection_images(
            &image_ids.into_iter()
                .map(|image_id| CollectionImageInsertRow {
                    collection_id: *collection_id,
                    image_id: **image_id
                })
                .collect::<Vec<_>>()
            )
        )?;
        debug!("Added {} images to collection: {:?}", inserted_images.len(), collection_id);
        Ok(inserted_images.len())
    }

    fn remove_images_from_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<usize, Error> {
        debug!("Removing {} images from collection for user: {}, collection_id: {}", image_ids.len(), user.id, *collection_id);
        let mut connection = self.get_connection()?;
        let deleted_images = connection.transaction(|conn| conn.collection_dao().delete_collection_images(
            *collection_id,
            &image_ids.iter().map(|image_id| **image_id).collect::<Vec<_>>()
        ))?;
        debug!("Deleted {} images from collection: {:?}", deleted_images.len(), collection_id);
        Ok(deleted_images.len())
    }
}
