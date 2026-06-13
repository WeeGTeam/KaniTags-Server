use kani_domain_api_incoming::collection_service::{AddImagesToCollectionError, CollectionService, CreateCollectionError, DeleteCollectionError, LoadCollectionsError, RemoveImagesFromCollectionError};
use kani_domain_api_model::collection::{Collection, CollectionId};
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::Database;
use std::sync::Arc;
use tracing::{error, info, warn};

pub struct CollectionServiceImpl {
    database: Arc<dyn Database + Sync + Send>,
}

impl CollectionServiceImpl {
    pub fn new(database: Arc<dyn Database + Sync + Send>) -> Self {
        Self { database }
    }
}

impl CollectionService for CollectionServiceImpl {
    fn load_collections_by_user(&self, user: &User) -> Result<Vec<Collection>, LoadCollectionsError> {
        info!("Loading collections for user");
        let collections = self.database.load_collections_by_user(user)?;
        info!("Loaded {} collections for user", collections.len());
        Ok(collections)
    }

    fn create_collection(&self, user: &User, collection_name: &str) -> Result<Collection, CreateCollectionError> {
        info!("Creating collection '{}' for user", collection_name);
        if let Some(existing_collection) = self.database.load_collection_by_user_and_name(user, collection_name)? {
            error!("Collection '{}' already exists for user", collection_name);
            return Err(CreateCollectionError::CollectionAlreadyExists(CollectionId(existing_collection.id)));
        }
        let collection = self.database.create_collection(user, collection_name)?;
        info!("Created collection '{}' for user", collection_name);
        Ok(collection)
    }

    fn delete_collection(&self, user: &User, collection_id: CollectionId) -> Result<(), DeleteCollectionError> {
        info!("Deleting collection {} for user", *collection_id);
        let collection = match self.database.load_collection_by_user_and_id(user, collection_id.clone())? {
            Some(collection) => collection,
            None => {
                error!("Collection with id '{}' does not exist for user", *collection_id);
                return Err(DeleteCollectionError::CollectionDoesNotExist(collection_id));
            }
        };
        self.database.delete_collection(user, collection_id.clone())?;
        info!("Deleted collection '{}' for user", &collection.name);
        Ok(())
    }

    fn add_images_to_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<(), AddImagesToCollectionError> {
        info!("Adding {} images to collection with id '{}' for user", image_ids.len(), *collection_id);
        let collection = match self.database.load_collection_by_user_and_id(user, collection_id.clone())? {
            Some(collection) => collection,
            None => {
                error!("Collection with id '{}' does not exist for user", *collection_id);
                return Err(AddImagesToCollectionError::CollectionDoesNotExist(collection_id));
            },
        };
        if image_ids.is_empty() {
            info!("No images to add to collection '{}'", &collection.name);
            return Ok(());
        }
        let accessible_images = self.database.get_images_by_image_ids(user, image_ids)?;
        if accessible_images.len() != image_ids.len() {
            error!("User does not have access to all images being added to collection '{}' or some images do not exist", &collection.name);
            return Err(AddImagesToCollectionError::InsufficientImageAccess(image_ids.to_vec()));
        }
        let added_count = self.database.add_images_to_collection(user, collection_id.clone(), &accessible_images.iter().map(|image| image.id.clone()).collect::<Vec<ImageId>>())?;
        if added_count != image_ids.len() {
            warn!("Not all images were added to collection '{}'", &collection.name);
        }
        info!("Added {} images to collection '{}' for user", added_count, &collection.name);
        Ok(())
    }

    fn remove_images_from_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<(), RemoveImagesFromCollectionError> {
        info!("Removing {} images from collection with id '{}' for user", image_ids.len(), *collection_id);
        let collection = match self.database.load_collection_by_user_and_id(user, collection_id.clone())? {
            Some(collection) => collection,
            None => {
                error!("Collection with id '{}' does not exist for user", *collection_id);
                return Err(RemoveImagesFromCollectionError::CollectionDoesNotExist(collection_id));
            },
        };
        let removed_count = self.database.remove_images_from_collection(user, collection_id.clone(), image_ids)?;
        if removed_count != image_ids.len() {
            warn!("Not all images were removed from collection '{}'", &collection.name);
        }
        info!("Removed {} images from collection '{}' for user", removed_count, &collection.name);
        Ok(())
    }
}
