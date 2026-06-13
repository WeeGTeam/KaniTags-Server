use crate::converter::{FromDomain, TryToDomain};
use kani_openapi::models::ImageId;
use std::num::ParseIntError;

impl FromDomain<kani_domain_api_model::image_id::ImageIdHash> for ImageId {
    fn from_domain(value: kani_domain_api_model::image_id::ImageIdHash) -> Self {
        ImageId(value.format_id_hash())
    }
}

impl TryToDomain<kani_domain_api_model::image_id::ImageId> for &ImageId {
    type Error = ImageIdConvertError;

    fn try_to_domain(self) -> Result<kani_domain_api_model::image_id::ImageId, Self::Error> {
        let id = self.0.parse().map_err(|e: ParseIntError| ImageIdConvertError::InvalidImageId(e.into()))?;
        Ok(kani_domain_api_model::image_id::ImageId(id))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ImageIdConvertError {
    #[error("invalid image id: {0}")]
    InvalidImageId(#[from] anyhow::Error),
}
