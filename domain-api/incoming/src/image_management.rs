use async_trait::async_trait;
use bytes::Bytes;
use thiserror::Error;

use kani_domain_api_model::image_format::{ImageFormat, ImageFormatError};
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::import::ImportSession;
use kani_domain_api_model::user::User;

#[async_trait]
pub trait ImageManagementService {
    async fn import_image(&self, user: &User, import_session_id: i64, image_name: String, image_data: Bytes) -> Result<(), ImportImageError>;

    async fn start_import_session(&self, user: &User) -> Result<ImportSession, StartImportSessionError>;

    async fn get_image(&self, image_id: ImageId) -> Result<(Bytes, ImageFormat), GetImageError>;
}

#[derive(Error, Debug)]
pub enum ImportImageError {
    #[error("Image import internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),

    #[error("Image has unsupported format: {0:?}")]
    UnsupportedImageFormat(Option<image::ImageFormat>),

    #[error("Unable to load Image: {0}")]
    InvalidImage(image::ImageError),

    #[error("Image already imported: {0}")]
    ImageAlreadyImported(ImageId),
}

#[derive(Error, Debug)]
pub enum StartImportSessionError {
    #[error("Import session internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum GetImageError {
    #[error("Get image internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),

    #[error("Image not found: {0}")]
    ImageNotFound(ImageId),
}


impl From<ImageFormatError> for ImportImageError {
    fn from(value: ImageFormatError) -> Self {
        match value {
            ImageFormatError::UnsupportedImageFormat(image_format) => ImportImageError::UnsupportedImageFormat(Some(image_format)),
        }
    }
}
