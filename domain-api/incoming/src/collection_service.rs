use kani_domain_api_model::collection::{Collection, CollectionId};
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::user::User;
use thiserror::Error;

pub trait CollectionService {
    fn load_collections_by_user(&self, user: &User) -> Result<Vec<Collection>, LoadCollectionsError>;

    fn create_collection(&self, user: &User, collection_name: &str) -> Result<Collection, CreateCollectionError>;

    fn delete_collection(&self, user: &User, collection_id: CollectionId) -> Result<(), DeleteCollectionError>;

    fn add_images_to_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<(), AddImagesToCollectionError>;

    fn remove_images_from_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<(), RemoveImagesFromCollectionError>;
}

#[derive(Error, Debug)]
pub enum LoadCollectionsError {
    #[error("Collection load internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum CreateCollectionError {
    #[error("Collection create internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
    #[error("Collection already exists: '{0:?}'")]
    CollectionAlreadyExists(CollectionId),
}

#[derive(Error, Debug)]
pub enum DeleteCollectionError {
    #[error("Collection delete internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
    #[error("Collection does not exist: '{0:?}'")]
    CollectionDoesNotExist(CollectionId),
}

#[derive(Error, Debug)]
pub enum AddImagesToCollectionError {
    #[error("Collection add images internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
    #[error("Collection does not exist: '{0:?}'")]
    CollectionDoesNotExist(CollectionId),
    #[error("User does not have access to all images being added to collection: '{0:?}'")]
    InsufficientImageAccess(Vec<ImageId>),
}

#[derive(Error, Debug)]
pub enum RemoveImagesFromCollectionError {
    #[error("Collection remove images internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
    #[error("Collection does not exist: '{0:?}'")]
    CollectionDoesNotExist(CollectionId),
}
