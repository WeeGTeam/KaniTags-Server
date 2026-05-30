use crate::image_format::ImageFormat;
use crate::image_hash::{IdHash, PerceptualHash};
use crate::image_id::ImageId;
use chrono::{DateTime, Utc};

#[derive (Clone)]
pub struct CreatePantsuImage {
    pub id_hash: IdHash,
    pub perceptual_hash: PerceptualHash,
    pub upload_filename: String,
    pub format: ImageFormat,
    pub dimensions: (u32, u32),
}

#[derive(Debug, Clone)]
pub struct PantsuImage {
    pub date_added: DateTime<Utc>,
    pub image_id: ImageId,
    pub format: ImageFormat,
    pub upload_filename: String,
    pub dimensions: (u32, u32),
}
