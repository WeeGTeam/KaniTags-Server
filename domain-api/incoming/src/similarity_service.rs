use kani_domain_api_model::image_id::ImageIdHash;
use kani_domain_api_model::similarity::SimilarImage;
use thiserror::Error;

pub trait SimilarityService {
    fn get_similar_images(&self, image_id_hash: &ImageIdHash) -> Result<Vec<SimilarImage>, GetSimilarImagesError>;
    fn calculate_similarity_groups(&self) -> Result<Vec<Vec<ImageIdHash>>, CalculateSimilarityGroupsError>;
}

#[derive(Error, Debug)]
pub enum GetSimilarImagesError {
    #[error("Get similar images internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum CalculateSimilarityGroupsError {
    #[error("Calculate similarity group internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
}
