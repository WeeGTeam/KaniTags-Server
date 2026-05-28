use crate::models::image::SimilarImagePairRow;
use kani_domain_api_model::similarity::{SimilarImage, SimilarImagePair};

impl Into<SimilarImage> for SimilarImagePairRow {
    fn into(self) -> SimilarImage {
        SimilarImage {
            image_id: self.id1,
            distance: self.dist
        }
    }
}

impl Into<SimilarImagePair> for SimilarImagePairRow {
    fn into(self) -> SimilarImagePair {
        SimilarImagePair {
            image_id1: self.id1,
            image_id2: self.id2,
            distance: self.dist
        }
    }
}
