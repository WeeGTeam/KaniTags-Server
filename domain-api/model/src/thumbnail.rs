pub enum ThumbnailKind {
    Gallery,
}

#[derive(Debug, Clone)]
pub struct ThumbnailOptions {
    pub max_size: u32,
    pub jpg_quality: u8,
}
