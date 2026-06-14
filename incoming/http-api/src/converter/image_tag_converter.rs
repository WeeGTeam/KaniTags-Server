use crate::converter::{FromDomain, ToDomain, TryToDomain};
use kani_domain_api_model::tag::image_tag::ImageTag;
use kani_domain_api_model::tag::{NewTag, TagName};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageTagConversionError {
    #[error("Image tag conversion error: '{0}'")]
    Unknown(#[from] anyhow::Error),
}

impl TryToDomain<NewTag> for &kani_openapi::models::NewImageTag {
    type Error = ImageTagConversionError;

    fn try_to_domain(self) -> Result<NewTag, Self::Error> {
        Ok(
            NewTag {
                tag_type: self.tag_type.to_domain(),
                tag_name: TagName::try_from(self.tag_name.to_owned())?,
            }
        )
    }
}

impl FromDomain<ImageTag> for kani_openapi::models::ImageTag {
    fn from_domain(image_tag: ImageTag) -> Self {
        Self {
            tag_id: image_tag.tag.id.to_string(),
            created_by: image_tag.user_id.map(|id| id.to_string()),
            created_at: image_tag.created_at,
        }
    }
}
