use anyhow::anyhow;


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
    type Error = anyhow::Error;
    fn try_from(format: image::ImageFormat) -> Result<Self, anyhow::Error> {
        Ok(match format {
            image::ImageFormat::Png => ImageFormat::PNG,
            image::ImageFormat::Jpeg => ImageFormat::JPG,
            _ => return Err(anyhow!("Unsupported image format {:?}", format))?
        })
    }
}
