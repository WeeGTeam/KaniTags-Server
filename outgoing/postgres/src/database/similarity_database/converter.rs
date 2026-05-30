use crate::models::image::SimilarImagePairRow;
use kani_domain_api_model::image_hash::IdHash;
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::similarity::{SimilarImage, SimilarImagePair};

impl Into<SimilarImage> for SimilarImagePairRow {
    fn into(self) -> SimilarImage {
        SimilarImage {
            image_id: ImageId(IdHash::try_from(self.id_hash2).unwrap()),
            distance: self.dist
        }
    }
}

impl Into<SimilarImagePair> for SimilarImagePairRow {
    fn into(self) -> SimilarImagePair {
        SimilarImagePair {
            image_id1: ImageId(IdHash::try_from(self.id_hash1).unwrap()),
            image_id2: ImageId(IdHash::try_from(self.id_hash2).unwrap()),
            distance: self.dist
        }
    }
}
