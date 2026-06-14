use crate::image_id::ImageIdHash;

#[derive(Debug)]
pub struct SimilarImage {
    pub image_id_hash: ImageIdHash,
    pub distance: i32,
}

#[derive(Debug)]
pub struct SimilarImagePair {
    pub image_id_hash1: ImageIdHash,
    pub image_id_hash2: ImageIdHash,
    pub distance: i32,
}
