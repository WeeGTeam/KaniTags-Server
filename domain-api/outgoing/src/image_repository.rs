use std::path::PathBuf;

use async_trait::async_trait;
use bytes::Bytes;
use kani_domain_api_model::image::PantsuImage;
use kani_domain_api_model::image_format::ImageFormat;
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::thumbnail::ThumbnailOptions;
use thiserror::Error;


#[async_trait]
pub trait ImageRepository {
    async fn store_image(
        &self,
        image_id: &ImageId,
        format: &ImageFormat,
        file_content: Bytes,
    ) -> Result<(), StoreImageError>;

    async fn store_jpg_thumbnail(
        &self,
        image_id: &ImageId,
        file_content: Bytes,
        options: ThumbnailOptions,
    ) -> Result<(), StoreImageError>;

    async fn load_image(
        &self,
        image: &PantsuImage,
    ) -> Result<Bytes, LoadImageError>;

    async fn load_jpg_thumbnail(
        &self,
        image_id: &ImageId,
        options: &ThumbnailOptions,
    ) -> Result<Bytes, LoadImageError>;
}

#[derive(Error, Debug)]
pub enum LoadImageError {
    #[error("Load image internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),

    #[error("Image not found in filesystem: '{0}'")]
    ImageNotFound(PathBuf),
}

#[derive(Error, Debug)]
pub enum StoreImageError {
    #[error("Store image internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),

    #[error("Image already exists in filesystem: '{0}'")]
    ImageAlreadyExists(PathBuf),
}
