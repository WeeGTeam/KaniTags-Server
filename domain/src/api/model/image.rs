use crate::image::image_format::ImageFormat;
use crate::image::image_id::ImageId;
use chrono::{DateTime, Utc};


#[derive (Clone)]
pub struct PantsuImage {
    pub id: ImageId,
    pub format: ImageFormat,
    pub _dimensions: (u32, u32),
    pub _date_added: DateTime<Utc>,
}

impl PantsuImage {
    pub fn filename(&self) -> String {
        format!("{}.{}", self.id.filename_format(), self.format.extension())
    }

    pub fn filename_with_custom_extension(&self, format: ImageFormat) -> String {
        format!("{}.{}", self.id.filename_format(), format.extension())
    }
}
