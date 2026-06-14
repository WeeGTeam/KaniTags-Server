use crate::image_format::ImageFormat;
use crate::image_hash::{IdHash, PerceptualHash};
use crate::image_id::{ImageId, ImageIdHash};
use bytes::Bytes;
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
    pub id: ImageId,
    pub date_added: DateTime<Utc>,
    pub image_id_hash: ImageIdHash,
    pub format: ImageFormat,
    pub upload_filename: String,
    pub dimensions: (u32, u32),
}

pub struct ImageDownloadData {
    pub bytes: Bytes,
    pub filename: String,
    pub format: ImageFormat
}
