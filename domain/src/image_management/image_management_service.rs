use anyhow::Context;
use async_trait::async_trait;
use bytes::Bytes;
use std::sync::Arc;
use tracing::{info, warn};

use crate::image::thumbnail::{create_thumbnail_in_memory, GALLERY_THUMBNAIL_OPTIONS};
use crate::image::try_create_pantsu_image;
use kani_domain_api_incoming::image_management::{ImageManagementService, ImportImageError, StartImportSessionError};
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::import::ImportSession;
use kani_domain_api_model::user::User;
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
        image_id: &ImageId,
        image_data: Bytes,
    ) -> Result<(), ImportImageError> {
        let thumbnail = create_thumbnail_in_memory(image_id.clone(), image_data, GALLERY_THUMBNAIL_OPTIONS).await?;
        self.image_repository.store_jpg_thumbnail(&image_id, thumbnail, GALLERY_THUMBNAIL_OPTIONS).await.map_err(|e| ImportImageError::Unknown(e.into()))
    }
}

#[async_trait]
impl ImageManagementService for ImageManagementServiceImpl {
    async fn import_image(
        &self,
        user: &User,
        import_session_id: i64,
        image_name: String,
        image_data: Bytes,
    ) -> Result<(), ImportImageError> {
        let image = try_create_pantsu_image(&image_name, &image_data)?;
        let image_id = ImageId(image.id_hash);
        // image_id::verify_image_id(&image_import.image_id, image.id())?;

        let db_image = self.database.get_image_by_image_id(&image_id)
            .context("Failed attempt to load image from database")?;
        if let Some(db_image) = db_image {
            return Err(ImportImageError::ImageAlreadyImported(db_image.image_id));
        }

        info!("Store image '{}' in library: '{}'", image_name, image_id.filename_format());
        allow_existing_image(self.image_repository.store_image(&image_id, image_data.clone()).await)?;
        let _ = self.create_thumbnail(&image_id, image_data).await.inspect_err(|e| warn!("Failed to create thumbnail: {}", e));

        let stored_image = self.database.store_image(&user, import_session_id, &image)?;
        info!("Stored image '{}' with id '{}'", image_name, stored_image.image_id);

        Ok(())
    }

    async fn start_import_session(&self, user: &User) -> Result<ImportSession, StartImportSessionError> {
        info!("Starting import session");
        let session = self.database.start_import_session(&user)?;
        info!("Started import session with id '{}'", session.id);
        Ok(session)
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
