use crate::converter::{FromDomain, TryToDomain};
use kani_domain_api_model::image::PantsuImage;
use kani_domain_api_model::image_format::ImageFormat;
use kani_openapi::models::{ImageDetailsDto, ImageDimensionDto, ImageId};
use std::num::ParseIntError;

impl FromDomain<kani_domain_api_model::image_id::ImageId> for ImageId {
    fn from_domain(image_id: kani_domain_api_model::image_id::ImageId) -> Self {
        ImageId(image_id.0.to_string())
    }
}

impl TryToDomain<kani_domain_api_model::image_id::ImageId> for &ImageId {
    type Error = ImageIdConvertError;

    fn try_to_domain(self) -> Result<kani_domain_api_model::image_id::ImageId, Self::Error> {
        let id = self.0.parse().map_err(|e: ParseIntError| ImageIdConvertError::InvalidImageId(e.into()))?;
        Ok(kani_domain_api_model::image_id::ImageId(id))
    }
}

impl FromDomain<PantsuImage> for ImageDetailsDto {

    fn from_domain(image: PantsuImage) -> Self {
        Self {
            id: image.id.0.to_string(),
            created_at: image.date_added,
            upload_filename: image.upload_filename,
            image_format: String::from_domain(image.format),
            image_dimension: ImageDimensionDto::from_domain(image.dimensions),
        }
    }
}

impl FromDomain<ImageFormat> for String {
    fn from_domain(format: ImageFormat) -> Self {
        match format {
            ImageFormat::JPG => "JPG".to_string(),
            ImageFormat::PNG => "PNG".to_string(),
        }
    }
}

impl FromDomain<(u32, u32)> for ImageDimensionDto {
    fn from_domain(dimensions: (u32, u32)) -> Self {
        Self {
            width: dimensions.0 as i32,
            height: dimensions.1 as i32,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ImageIdConvertError {
    #[error("invalid image id: {0}")]
    InvalidImageId(#[from] anyhow::Error),
}
