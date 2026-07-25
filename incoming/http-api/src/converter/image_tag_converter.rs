use crate::converter::{FromDomain, ToDomain, TryToDomain};
use kani_domain_api_model::tag::image_tag::ImageTag;
use kani_domain_api_model::tag::tag_source_site::TagSourceSite;
use kani_domain_api_model::tag::{NewTag, TagName};
use kani_openapi::models::Tag;
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
            tag: Tag::from_domain(image_tag.tag),
            created_by_user: image_tag.user_id.map(|id| id.to_string()),
            created_by_source_site: FromDomain::from_domain(image_tag.source_site),
            created_at: image_tag.created_at,
        }
    }
}

impl FromDomain<TagSourceSite> for kani_openapi::models::TagSourceSite {
    fn from_domain(tag_source_site: TagSourceSite) -> Self {
        match tag_source_site {
            TagSourceSite::Gelbooru => kani_openapi::models::TagSourceSite::Gelbooru,
        }
    }
}
