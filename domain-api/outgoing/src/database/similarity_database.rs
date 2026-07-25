use kani_domain_api_model::image_id::ImageIdHash;
use kani_domain_api_model::similarity::{SimilarImage, SimilarImagePair};

#[cfg_attr(feature = "test-util", mockall::automock)]
pub trait SimilarityDatabase {
    fn get_similar_images(&self, image_id_hash: &ImageIdHash) -> Result<Vec<SimilarImage>, anyhow::Error>;
    fn get_all_similar_images(&self) -> Result<Vec<SimilarImagePair>, anyhow::Error>;
}
