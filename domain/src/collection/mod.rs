use kani_domain_api_incoming::collection_service::{AddImagesToCollectionError, CollectionService, CreateCollectionError, DeleteCollectionError, LoadCollectionsError, RemoveImagesFromCollectionError};
use kani_domain_api_model::collection::{Collection, CollectionId, CollectionName};
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::Database;
use kani_domain_api_outgoing::database::error::{ReadDbError, WriteDbError};
use std::ops::Deref;
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

#[async_trait::async_trait]
impl CollectionService for CollectionServiceImpl {
    async fn load_collections_by_user(&self, user: &User) -> Result<Vec<Collection>, LoadCollectionsError> {
        info!("Loading collections for user");
        let collections = self.database.collection().load_collections_by_user(user).await
            .map_err(|e| LoadCollectionsError::Unknown(e.into()))?;
        info!("Loaded {} collections for user", collections.len());
        Ok(collections)
    }

    async fn create_collection(&self, user: &User, collection_name: &CollectionName) -> Result<Collection, CreateCollectionError> {
        info!("Creating collection '{}' for user", collection_name.deref());
        match self.database.collection().create_collection(user, collection_name).await {
            Ok(collection) => {
                info!("Created collection '{}' for user", collection_name.deref());
                Ok(collection)
            }
            Err(WriteDbError::UniqueViolation) => {
                error!("Collection '{}' already exists for user", collection_name.deref());
                match self.database.collection().load_collection_by_user_and_name(user, collection_name).await
                {
                    Ok(existing) => Err(CreateCollectionError::CollectionAlreadyExists(existing.id)),
                    Err(e) => Err(CreateCollectionError::Unknown(e.into())),
                }
            }
            Err(e) => Err(CreateCollectionError::Unknown(e.into())),
        }
    }

    async fn delete_collection(&self, user: &User, collection_id: CollectionId) -> Result<(), DeleteCollectionError> {
        info!("Deleting collection {} for user", *collection_id);
        match self.database.collection().delete_collection(user, collection_id).await {
            Ok(()) => {
                info!("Deleted collection '{}' for user", *collection_id);
                Ok(())
            }
            Err(ReadDbError::NotFound) => {
                error!("Collection with id '{}' does not exist for user", *collection_id);
                Err(DeleteCollectionError::CollectionDoesNotExist(collection_id))
            }
            Err(e) => Err(DeleteCollectionError::Unknown(e.into())),
        }
    }

    async fn add_images_to_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<(), AddImagesToCollectionError> {
        info!("Adding {} images to collection with id '{}' for user", image_ids.len(), *collection_id);
        let collection = match self.database.collection().load_collection_by_user_and_id(user, collection_id).await {
            Ok(collection) => collection,
            Err(ReadDbError::NotFound) => {
                error!("Collection with id '{}' does not exist for user", *collection_id);
                return Err(AddImagesToCollectionError::CollectionDoesNotExist(collection_id));
            },
            Err(e) => return Err(AddImagesToCollectionError::Unknown(e.into())),
        };
        let mut requested: Vec<ImageId> = image_ids.to_vec();
        requested.sort_by_key(|id| **id);
        requested.dedup_by_key(|id| **id);
        if requested.is_empty() {
            info!("No images to add to collection '{}'", &collection.name.deref());
            return Ok(());
        }
        let accessible_images = self.database.image().get_images_by_image_ids(user, &requested).await?;
        if accessible_images.len() != requested.len() {
            error!("User does not have access to all images being added to collection '{}' or some images do not exist", &collection.name.deref());
            return Err(AddImagesToCollectionError::InsufficientImageAccess(requested));
        }
        let added_count = self.database.collection().add_images_to_collection(user, collection_id, &accessible_images.iter().map(|image| image.id).collect::<Vec<ImageId>>()).await
            .map_err(|e| AddImagesToCollectionError::Unknown(e.into()))?;
        if added_count != requested.len() {
            warn!("Not all images were added to collection '{}'", &collection.name.deref());
        }
        info!("Added {} images to collection '{}' for user", added_count, &collection.name.deref());
        Ok(())
    }

    async fn remove_images_from_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<(), RemoveImagesFromCollectionError> {
        info!("Removing {} images from collection with id '{}' for user", image_ids.len(), *collection_id);
        let collection = match self.database.collection().load_collection_by_user_and_id(user, collection_id).await {
            Ok(collection) => collection,
            Err(ReadDbError::NotFound) => {
                error!("Collection with id '{}' does not exist for user", *collection_id);
                return Err(RemoveImagesFromCollectionError::CollectionDoesNotExist(collection_id));
            },
            Err(e) => return Err(RemoveImagesFromCollectionError::Unknown(e.into())),
        };
        let removed_count = self.database.collection().remove_images_from_collection(user, collection_id, image_ids).await
            .map_err(|e| RemoveImagesFromCollectionError::Unknown(e.into()))?;
        if removed_count != image_ids.len() {
            warn!("Not all images were removed from collection '{}'", &collection.name.deref());
        }
        info!("Removed {} images from collection '{}' for user", removed_count, &collection.name.deref());
        Ok(())
    }
}
