use kani_domain_api_incoming::image_search_service::{GetImageError, ImageSearchService, SearchImagesError};
use kani_domain_api_model::image::PantsuImage;
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::image_search::ImageSearchFilter;
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::ImageDatabase;
use std::sync::Arc;
use tracing::info;

pub struct ImageSearchServiceImpl {
    database: Arc<dyn ImageDatabase + Sync + Send>,
}

impl ImageSearchServiceImpl {
    pub fn new(database: Arc<dyn ImageDatabase + Send + Sync>) -> Self {
        Self { database }
    }
}

#[async_trait::async_trait]
impl ImageSearchService for ImageSearchServiceImpl {
    async fn search_images(&self, user: &User, filter: ImageSearchFilter) -> Result<Vec<ImageId>, SearchImagesError> {
        info!("searching images with filter '{:?}'", filter);
        Ok(self.database.search_images(user, filter).await?)
    }

    async fn get_image(&self, user: &User, image_id: ImageId) -> Result<PantsuImage, GetImageError> {
        info!("getting image with id '{:?}'", image_id);
        match self.database.get_image_by_image_id(&user, image_id).await? {
            Some(image) => Ok(image),
            None => {
                info!("Image does not exist: {}", *image_id);
                Err(GetImageError::ImageNotFound(image_id))
            }
        }
    }
}
