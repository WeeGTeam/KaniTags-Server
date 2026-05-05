use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use tracing::{info, warn};

use crate::api::incoming::image_management::{ImageManagementService, ImportImageError};
use crate::api::model::image::PantsuImage;
use crate::api::outgoing::image_repository::{ImageRepository, StoreImageError};
use crate::image::try_create_pantsu_image;
use crate::image::thumbnail::{GALLERY_THUMBNAIL_OPTIONS, create_thumbnail_in_memory};


pub struct ImageManagementServiceImpl {
    image_repository: Arc<dyn ImageRepository + Sync + Send>,
}

impl ImageManagementServiceImpl {
    pub fn new(image_repository: Arc<dyn ImageRepository + Send + Sync + 'static>) -> Self {
        Self {
            image_repository,
        }
    }

    async fn create_thumbnail(
        &self,
        image: &PantsuImage,
        image_data: Bytes,
    ) -> Result<(), ImportImageError> {
        let thumbnail = create_thumbnail_in_memory(image.id.clone(), image_data, GALLERY_THUMBNAIL_OPTIONS).await?;
        self.image_repository.store_jpg_thumbnail(&image, thumbnail, GALLERY_THUMBNAIL_OPTIONS).await.map_err(|e| ImportImageError::Unknown(e.into()))
    }
}

#[async_trait]
impl ImageManagementService for ImageManagementServiceImpl {
     async fn import_image(
        &self,
        image_name: String,
        image_data: Bytes,
    ) -> Result<(), ImportImageError> {
        let image = try_create_pantsu_image(&image_data)?;
        // image_id::verify_image_id(&image_import.image_id, image.id())?;

        // TODO: import: check if file exists (in db)

        info!("Store image '{}' in library: '{}'", image_name, image.filename());
        allow_existing_image(self.image_repository.store_image(image.clone(), image_data.clone()).await)?;
        let _ = self.create_thumbnail(&image, image_data).await.inspect_err(|e| warn!("Failed to create thumbnail: {}", e));


        // TODO: add to db

        Ok(())
    }
}

fn allow_existing_image(store_result: Result<(), StoreImageError>) -> Result<(), ImportImageError> {
    match store_result {
        Ok(it) => Ok(it),
        Err(unknown @ StoreImageError::Unknown(_)) => Err(ImportImageError::Unknown(unknown.into())),
        Err(StoreImageError::ImageAlreadyExists(e)) => {
            warn!("Failed to store image: {}", e.display());
            Ok(())
        },
    }
}