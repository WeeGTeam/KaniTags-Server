use kani_domain_api_model::image_id::ImageId;
use thiserror::Error;

pub trait SimilarityService {
    fn calculate_similarity_groups(&self) -> Result<Vec<Vec<ImageId>>, CalculateSimilarityGroupsError>;
}

#[derive(Error, Debug)]
pub enum CalculateSimilarityGroupsError {
    #[error("Calculate similarity group internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
}
