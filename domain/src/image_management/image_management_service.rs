use anyhow::Context;
use async_trait::async_trait;
use bytes::Bytes;
use std::sync::Arc;
use tracing::{info, warn};

use crate::image::thumbnail::{create_thumbnail_in_memory, GALLERY_THUMBNAIL_OPTIONS};
use crate::image::try_create_pantsu_image;
use kani_domain_api_incoming::image_management::{ImageManagementService, ImportImageError};
use kani_domain_api_model::image::CreatePantsuImage;
use kani_domain_api_outgoing::database::Database;
use kani_domain_api_outgoing::image_repository::{ImageRepository, StoreImageError};


pub struct ImageManagementServiceImpl {
    image_repository: Arc<dyn ImageRepository + Sync + Send>,
    database: Arc<dyn Database + Sync + Send>,
}

impl ImageManagementServiceImpl {
    pub fn new(
        image_repository: Arc<dyn ImageRepository + Send + Sync + 'static>,
        database: Arc<dyn Database + Send + Sync + 'static>,
    ) -> Self {
        Self {
            image_repository,
            database,
        }
    }

    async fn create_thumbnail(
        &self,
        image: &CreatePantsuImage,
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
        let image = try_create_pantsu_image(&image_name, &image_data)?;
        // image_id::verify_image_id(&image_import.image_id, image.id())?;

        let db_image = self.database.get_image_by_id_hash(image.id.get_id_hash())
            .context("Failed attempt to load image from database")?;
        if let Some(db_image) = db_image {
            return Err(ImportImageError::ImageAlreadyImported(db_image.id));
        }

        info!("Store image '{}' in library: '{}'", image_name, image.filename());
        allow_existing_image(self.image_repository.store_image(image.clone(), image_data.clone()).await)?;
        let _ = self.create_thumbnail(&image, image_data).await.inspect_err(|e| warn!("Failed to create thumbnail: {}", e));

        self.database.store_image(&image)?;

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
