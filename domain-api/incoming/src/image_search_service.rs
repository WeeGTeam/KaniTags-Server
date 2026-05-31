use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::image_search::ImageSearchFilter;
use thiserror::Error;

pub trait ImageSearchService {
    fn search_images(&self, filter: &ImageSearchFilter) -> Result<Vec<ImageId>, SearchImagesError>;
}

#[derive(Error, Debug)]
pub enum SearchImagesError {
    #[error("Search images internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
}
