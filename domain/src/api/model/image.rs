use crate::api::model::{image_format::ImageFormat, image_id::ImageId};
use chrono::{DateTime, Utc};


#[derive (Clone)]
pub struct CreatePantsuImage {
    pub id: ImageId,
    pub format: ImageFormat,
    pub _dimensions: (u32, u32),
    pub _date_added: DateTime<Utc>,
}

impl CreatePantsuImage {
    pub fn filename(&self) -> String {
        format!("{}.{}", self.id.filename_format(), self.format.extension())
    }

    pub fn filename_with_custom_extension(&self, format: ImageFormat) -> String {
        format!("{}.{}", self.id.filename_format(), format.extension())
    }
}
