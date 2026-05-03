use async_trait::async_trait;
use bytes::Bytes;

use crate::api::model::image::PantsuImage;
use crate::api::model::thumbnail::ThumbnailOptions;
use crate::common::result::Result;


#[async_trait]
pub trait ImageRepository {
    async fn store_image(&self, image: PantsuImage, file_content: Bytes) -> Result<()>;
    
    async fn store_jpg_thumbnail(
        &self,
        image: &PantsuImage,
        file_content: Bytes,
        options: ThumbnailOptions,
    ) -> Result<()>;
}