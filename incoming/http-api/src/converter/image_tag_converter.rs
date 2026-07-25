use crate::converter::{FromDomain, ToDomain, TryToDomain};
use kani_domain_api_model::tag::image_tag::ImageTag;
use kani_domain_api_model::tag::image_tag_source_site::ImageTagSourceSite;
use kani_domain_api_model::tag::{NewTag, TagName};
use kani_openapi::models::{ImageTagDto, ImageTagSourceSiteDto, NewImageTagDto, TagDto};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageTagConversionError {
    #[error("Image tag conversion error: '{0}'")]
    Unknown(#[from] anyhow::Error),
}

impl TryToDomain<NewTag> for &NewImageTagDto {
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

impl FromDomain<ImageTag> for ImageTagDto {
    fn from_domain(image_tag: ImageTag) -> Self {
        Self {
            tag: TagDto::from_domain(image_tag.tag),
            created_by_user: image_tag.user_id.map(|id| id.to_string()),
            created_by_source_site: FromDomain::from_domain(image_tag.source_site),
            created_at: image_tag.created_at,
        }
    }
}

impl FromDomain<ImageTagSourceSite> for ImageTagSourceSiteDto {
    fn from_domain(image_tag_source_site: ImageTagSourceSite) -> Self {
        match image_tag_source_site {
            ImageTagSourceSite::Gelbooru => ImageTagSourceSiteDto::Gelbooru,
        }
    }
}
