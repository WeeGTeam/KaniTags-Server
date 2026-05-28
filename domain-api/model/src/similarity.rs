
#[derive(Debug)]
pub struct SimilarImage {
    pub image_id: i64,
    pub distance: i32,
}

#[derive(Debug)]
pub struct SimilarImagePair {
    pub image_id1: i64,
    pub image_id2: i64,
    pub distance: i32,
}
