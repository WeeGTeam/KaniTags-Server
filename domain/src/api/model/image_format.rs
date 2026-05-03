use crate::common::error::Error;
use crate::common::result::Result;

#[derive (Clone)]
pub enum ImageFormat {
    PNG, JPG,
}

impl ImageFormat {
    pub fn extension(&self) -> String {
        match self {
            ImageFormat::PNG => "png".to_string(),
            ImageFormat::JPG => "jpg".to_string(),
        }
    }
}

impl TryFrom<image::ImageFormat> for ImageFormat {
    type Error = Error;
    fn try_from(format: image::ImageFormat) -> Result<Self> {
        Ok(match format {
            image::ImageFormat::Png => ImageFormat::PNG,
            image::ImageFormat::Jpeg => ImageFormat::JPG,
            _ => return Err(Error::TodoError())
        })
    }
}
