use crate::models::image::SimilarImagePairRow;
use kani_domain_api_model::image_hash::{hash_to_hex, IdHash};
use kani_domain_api_model::image_id::ImageIdHash;
use kani_domain_api_model::similarity::{SimilarImage, SimilarImagePair};

impl TryInto<SimilarImage> for SimilarImagePairRow {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<SimilarImage, Self::Error> {
        Ok(SimilarImage {
            image_id_hash: ImageIdHash(IdHash::try_from(self.id_hash2).map_err(|h| anyhow::anyhow!("Invalid id_hash2 '{}'", hash_to_hex(&h)))?),
            distance: self.dist
        })
    }

}

impl TryInto<SimilarImagePair> for SimilarImagePairRow {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<SimilarImagePair, Self::Error> {
        Ok(SimilarImagePair {
            image_id_hash1: ImageIdHash(IdHash::try_from(self.id_hash1).map_err(|h| anyhow::anyhow!("Invalid id_hash1 '{}'", hash_to_hex(&h)))?),
            image_id_hash2: ImageIdHash(IdHash::try_from(self.id_hash2).map_err(|h| anyhow::anyhow!("Invalid id_hash2 '{}'", hash_to_hex(&h)))?),
            distance: self.dist
        })
    }
}
