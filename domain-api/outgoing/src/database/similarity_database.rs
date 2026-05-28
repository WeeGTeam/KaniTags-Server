use kani_domain_api_model::similarity::{SimilarImage, SimilarImagePair};

pub trait SimilarityDatabase {
    fn get_similar_images(&self, image_id: i64) -> Result<Vec<SimilarImage>, anyhow::Error>;
    fn get_all_similar_images(&self) -> Result<Vec<SimilarImagePair>, anyhow::Error>;
}
