use anyhow::Context;
use async_trait::async_trait;
use bytes::Bytes;
use std::sync::Arc;
use tracing::{info, warn};

use crate::image::thumbnail::{create_thumbnail_in_memory, get_thumbnail_options};
use crate::image::try_create_pantsu_image;
use kani_domain_api_incoming::image_management::{CloseImportSessionError, GetImageError, GetImportSessionsError, ImageManagementService, ImportImageError, StartImportSessionError};
use kani_domain_api_model::image::{ImageDownloadData, PantsuImage};
use kani_domain_api_model::image_format::ImageFormat;
use kani_domain_api_model::image_id::{ImageId, ImageIdHash};
use kani_domain_api_model::import::{ImportSession, ImportSessionId};
use kani_domain_api_model::thumbnail::ThumbnailKind;
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::Database;
use kani_domain_api_outgoing::image_repository::{ImageRepository, LoadImageError, StoreImageError};


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
        image_hash_id: &ImageIdHash,
        image_data: Bytes,
        kind: &ThumbnailKind,
    ) -> Result<Bytes, anyhow::Error> {
        let options = get_thumbnail_options(&kind);
        let thumbnail = create_thumbnail_in_memory(image_hash_id.clone(), image_data, options.clone()).await?;
        self.image_repository.store_jpg_thumbnail(&image_hash_id, thumbnail.clone(), options).await?;
        Ok(thumbnail)
    }

    async fn load_image_bytes(&self, image: &PantsuImage) -> Result<Bytes, GetImageError> {
        self.image_repository
            .load_image(&image).await
            .map_err(|e| match e {
                LoadImageError::ImageNotFound(_) => GetImageError::ImageNotFound(image.id.clone()),
                unknown @ LoadImageError::Unknown(_) => GetImageError::Unknown(unknown.into()),
            })
    }
}

#[async_trait]
impl ImageManagementService for ImageManagementServiceImpl {
    async fn import_image(
        &self,
        user: &User,
        import_session_id: ImportSessionId,
        image_name: String,
        image_data: Bytes,
    ) -> Result<(), ImportImageError> {
        let image = try_create_pantsu_image(&image_name, &image_data)?;
        let image_id_hash = ImageIdHash(image.id_hash);
        let import_session = self.database.get_import_session_by_id_and_user(&user, import_session_id.clone())?
            .ok_or_else(|| ImportImageError::MissingImportSession(import_session_id.clone()))?;

        if import_session.closed_at.is_some() {
            return Err(ImportImageError::ImportSessionClosed(ImportSessionId(import_session.id)));
        }

        let db_image = self.database.get_image_by_image_id_hash(&image_id_hash)
            .context("Failed attempt to load image from database")?;
        if let Some(db_image) = db_image {
            return Err(ImportImageError::ImageAlreadyImported(db_image.image_id_hash));
        }

        info!("Store image '{}' in library", image_id_hash);
        allow_existing_image(self.image_repository.store_image(&image_id_hash, &image.format, image_data.clone()).await)?;
        let _ = self.create_thumbnail(&image_id_hash, image_data, &ThumbnailKind::Gallery).await.inspect_err(|e| warn!("Failed to create thumbnail: {}", e));

        let stored_image = self.database.store_image(&user, ImportSessionId(import_session.id), &image)?;
        info!("Stored image '{}' with id '{}'", image_name, stored_image.image_id_hash);

        Ok(())
    }

    async fn start_import_session(&self, user: &User) -> Result<ImportSessionId, StartImportSessionError> {
        info!("Starting import session");
        let session = self.database.start_import_session(&user)?;
        info!("Started import session with id '{}'", *session);
        Ok(session)
    }

    async fn close_import_session(&self, user: &User, import_session_id: ImportSessionId) -> Result<(), CloseImportSessionError> {
        info!("Closing import session with id '{}'", *import_session_id);
        let import_session = self.database.get_import_session_by_id_and_user(&user, import_session_id.clone())?
            .ok_or_else(|| CloseImportSessionError::ImportSessionMissing(import_session_id.clone()))?;
        if import_session.closed_at.is_some() {
            return Err(CloseImportSessionError::ImportSessionClosed(import_session_id));
        }
        self.database.close_import_session(ImportSessionId(import_session.id))?;
        info!("Closed import session with id '{}'", import_session.id);
        Ok(())
    }


    async fn get_import_sessions(&self, user: &User) -> Result<Vec<ImportSession>, GetImportSessionsError> {
        info!("Getting import sessions for user '{}'", user.user_name);
        let sessions = self.database.get_import_sessions(&user)?;
        info!("Retrieved {} import sessions for user '{}'", sessions.len(), user.user_name);
        Ok(sessions)
    }


    async fn get_image(&self, image_id: ImageId) -> Result<ImageDownloadData, GetImageError> {
        let db_image = self.database
            .get_image_by_image_id(image_id.clone())?
            .ok_or_else(|| GetImageError::ImageNotFound(image_id.clone()))?;

        let loaded_image = self.load_image_bytes(&db_image).await?;

        Ok(ImageDownloadData {
            bytes: loaded_image,
            filename: db_image.image_id_hash.format_id_hash(),
            format: db_image.format
        })
    }

    async fn get_thumbnail(&self, image_id: ImageId, kind: ThumbnailKind) -> Result<ImageDownloadData, GetImageError> {
        let db_image = self.database
            .get_image_by_image_id(image_id.clone())?
            .ok_or_else(|| GetImageError::ImageNotFound(image_id.clone()))?;
        let thumbnail_options = get_thumbnail_options(&kind);
        match self.image_repository.load_jpg_thumbnail(&db_image.image_id_hash, &thumbnail_options).await {
            Ok(loaded_thumbnail) => Ok(ImageDownloadData {
                bytes: loaded_thumbnail,
                filename: db_image.image_id_hash.format_id_hash(),
                format: ImageFormat::JPG
            }),
            Err(LoadImageError::ImageNotFound(_)) => {
                info!("Thumbnail for image '{:?}' not found, creating it", image_id);
                let loaded_image = self.load_image_bytes(&db_image).await?;
                let thumbnail = self.create_thumbnail(&db_image.image_id_hash, loaded_image, &kind).await.map_err(|e| GetImageError::Unknown(e))?;
                Ok(ImageDownloadData {
                    bytes: thumbnail,
                    filename: db_image.image_id_hash.format_id_hash(),
                    format: ImageFormat::JPG
                })
            }
            Err(unknown @ LoadImageError::Unknown(_)) => Err(GetImageError::Unknown(unknown.into())),
        }
    }
}

fn allow_existing_image(store_result: Result<(), StoreImageError>) -> Result<(), ImportImageError> {
    match store_result {
        Ok(it) => Ok(it),
        Err(unknown @ StoreImageError::Unknown(_)) => Err(ImportImageError::Unknown(unknown.into())),
        Err(e @ StoreImageError::ImageAlreadyExists(_)) => {
            warn!("Failed to store image: {}", e);
            Ok(())
        },
    }
}
