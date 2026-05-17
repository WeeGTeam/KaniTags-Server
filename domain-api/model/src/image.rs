use crate::image_format::ImageFormat;
use crate::image_id::ImageId;
use chrono::{DateTime, Utc};

#[derive (Clone)]
pub struct CreatePantsuImage {
    pub id: ImageId,
    pub upload_filename: String,
    pub format: ImageFormat,
    pub dimensions: (u32, u32),
}

impl CreatePantsuImage {
    pub fn filename(&self) -> String {
        format!("{}.{}", self.id.filename_format(), self.format.extension())
    }

    pub fn filename_with_custom_extension(&self, format: ImageFormat) -> String {
        format!("{}.{}", self.id.filename_format(), format.extension())
    }
}

#[derive(Debug, Clone)]
pub struct PantsuImage {
    pub id: i64,
    pub date_added: DateTime<Utc>,
    pub image_id: ImageId,
    pub format: ImageFormat,
    pub upload_filename: String,
    pub dimensions: (u32, u32),
}
