use thiserror::Error;


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
    type Error = ImageFormatError;
    fn try_from(format: image::ImageFormat) -> Result<Self, ImageFormatError> {
        Ok(match format {
            image::ImageFormat::Png => ImageFormat::PNG,
            image::ImageFormat::Jpeg => ImageFormat::JPG,
            other => return Err(ImageFormatError::UnsupportedImageFormat(other))?
        })
    }
}

#[derive(Error, Debug)]
pub enum ImageFormatError {
    #[error("Unsupported image format: {0:?}")]
    UnsupportedImageFormat(image::ImageFormat),
}