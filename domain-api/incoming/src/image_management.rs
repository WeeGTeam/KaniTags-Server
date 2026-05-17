use async_trait::async_trait;
use bytes::Bytes;
use thiserror::Error;

use kani_domain_api_model::image_format::ImageFormatError;


#[async_trait]
pub trait ImageManagementService {
    async fn import_image(&self, image_name: String, image_data: Bytes) -> Result<(), ImportImageError>;
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

impl From<ImageFormatError> for ImportImageError {
    fn from(value: ImageFormatError) -> Self {
        match value {
            ImageFormatError::UnsupportedImageFormat(image_format) => ImportImageError::UnsupportedImageFormat(Some(image_format)),
        }
    }
}
