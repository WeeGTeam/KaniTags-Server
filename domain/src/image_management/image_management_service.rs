use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use tracing::info;

use crate::{common::error::Error, image::PantsuImage, library::LibraryService, api::ImageManagementService};


pub struct ImageManagementServiceImpl {
    fs_service: Arc<dyn LibraryService + Sync + Send>,
}

impl ImageManagementServiceImpl {
    pub fn new(fs_service: Arc<dyn LibraryService + Send + Sync + 'static>) -> Self {
        Self {
            fs_service: fs_service,
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
        self.fs_service.store_image(image.clone(), image_data).await?;

        // TODO: add to db

        Ok(())
    }
}