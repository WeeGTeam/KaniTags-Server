use std::path::PathBuf;

use async_trait::async_trait;
use bytes::Bytes;
use thiserror::Error;

use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::thumbnail::ThumbnailOptions;


#[async_trait]
pub trait ImageRepository {
    async fn store_image(&self, image_id: &ImageId, file_content: Bytes) -> Result<(), StoreImageError>;

    async fn store_jpg_thumbnail(
        &self,
        image_id: &ImageId,
        file_content: Bytes,
        options: ThumbnailOptions,
    ) -> Result<(), StoreImageError>;
}

#[derive(Error, Debug)]
pub enum StoreImageError {
    #[error("Store image internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),

    #[error("Image already exists in filesystem: '{0}'")]
    ImageAlreadyExists(PathBuf),
}
