use kani_domain_api_model::image_id::ImageIdHash;
use kani_domain_api_model::image_search::ImageSearchFilter;
use kani_domain_api_model::user::User;
use thiserror::Error;

pub trait ImageSearchService {
    fn search_images(&self, user: &User, filter: &ImageSearchFilter) -> Result<Vec<ImageIdHash>, SearchImagesError>;
}

#[derive(Error, Debug)]
pub enum SearchImagesError {
    #[error("Search images internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
}
