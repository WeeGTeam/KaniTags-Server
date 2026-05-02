use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use tracing::info;

use crate::{api::{incoming::ImageManagementService, outgoing::ImageRepository}, common::error::Error, image::{PantsuImage, thumbnail::create_gallery_thumbnail}};


pub struct ImageManagementServiceImpl {
    image_repository: Arc<dyn ImageRepository + Sync + Send>,
}

impl ImageManagementServiceImpl {
    pub fn new(image_repository: Arc<dyn ImageRepository + Send + Sync + 'static>) -> Self {
        Self {
            image_repository,
        }
    }
}

#[async_trait]
impl ImageManagementService for ImageManagementServiceImpl {
     async fn import_image(
        &self,
        image_name: String,
        image_data: Bytes,
    ) -> Result<(), Error> {
        let image = PantsuImage::try_from(image_data.as_ref())?;
        // image_id::verify_image_id(&image_import.image_id, image.id())?;

        // TODO: import: check if file exists (in db)

        info!("Store image '{}' in library: '{}'", image_name, image.filename());
        self.image_repository.store_image(image.clone(), image_data.clone()).await?;
        let thumbnail = create_gallery_thumbnail(image.id().clone(), image_data).await?;
        self.image_repository.store_jpg_thumbnail(&image, thumbnail).await?;

        // TODO: add to db

        Ok(())
    }
}