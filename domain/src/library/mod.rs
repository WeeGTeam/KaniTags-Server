use crate::common::result::Result;
use crate::image::PantsuImage;
use async_trait::async_trait;
use bytes::Bytes;

pub const GALLERY_THUMBNAIL_OPTIONS: ThumbnailOptions = ThumbnailOptions {
    max_size: 512,
    jpg_quality: 80,
};

pub struct ThumbnailOptions {
    pub max_size: u32,
    pub jpg_quality: u8,
}

#[async_trait]
pub trait Library {
    async fn store_image(&self, image: &PantsuImage, file_content: Bytes) -> Result<()>;

    async fn create_thumbnail(
        &self,
        image: &PantsuImage,
        file_content: Bytes,
    ) -> Result<()>;
}


