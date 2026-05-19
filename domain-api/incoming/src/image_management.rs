use async_trait::async_trait;
use bytes::Bytes;
use thiserror::Error;

use kani_domain_api_model::image_format::ImageFormatError;
use kani_domain_api_model::import::ImportSession;
use kani_domain_api_model::user::User;

#[async_trait]
pub trait ImageManagementService {
    async fn import_image(&self, user: &User, import_session_id: i64, image_name: String, image_data: Bytes) -> Result<(), ImportImageError>;

    async fn start_import_session(&self, user: &User) -> Result<ImportSession, StartImportSessionError>;
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
    ImageAlreadyImported(i64),
}

#[derive(Error, Debug)]
pub enum StartImportSessionError {
    #[error("Import session internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
}

impl From<ImageFormatError> for ImportImageError {
    fn from(value: ImageFormatError) -> Self {
        match value {
            ImageFormatError::UnsupportedImageFormat(image_format) => ImportImageError::UnsupportedImageFormat(Some(image_format)),
        }
    }
}
