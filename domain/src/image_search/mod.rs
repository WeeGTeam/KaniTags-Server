use kani_domain_api_incoming::image_search_service::{ImageSearchService, SearchImagesError};
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::image_search::ImageSearchFilter;
use kani_domain_api_outgoing::database::ImageDatabase;
use std::str::FromStr;
use std::sync::Arc;

pub struct ImageSearchServiceImpl {
    database: Arc<dyn ImageDatabase + Sync + Send>,
}

impl ImageSearchServiceImpl {
    pub fn new(database: Arc<dyn ImageDatabase + Send + Sync>) -> Self {
        Self { database }
    }
}

impl ImageSearchService for ImageSearchServiceImpl {
    fn search_images(&self, filter: &ImageSearchFilter) -> Result<Vec<ImageId>, SearchImagesError> {
        Ok(vec![ImageId::from_str("3b6368639f3e17fa")?])
    }
}
