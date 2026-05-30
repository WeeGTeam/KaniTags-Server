use crate::image_id::ImageId;

#[derive(Debug)]
pub struct SimilarImage {
    pub image_id: ImageId,
    pub distance: i32,
}

#[derive(Debug)]
pub struct SimilarImagePair {
    pub image_id1: ImageId,
    pub image_id2: ImageId,
    pub distance: i32,
}
