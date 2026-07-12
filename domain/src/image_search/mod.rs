use kani_domain_api_incoming::image_search_service::{ImageSearchService, SearchImagesError};
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

impl ImageSearchService for ImageSearchServiceImpl {
    fn search_images(&self, user: &User, filter: &ImageSearchFilter) -> Result<Vec<ImageId>, SearchImagesError> {
        info!("searching images with filter '{:?}'", filter);
        Ok(self.database.search_images(user, filter)?)
    }
}
