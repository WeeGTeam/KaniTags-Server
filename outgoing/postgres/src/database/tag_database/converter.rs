use crate::database::converter::{FromDomain, ToDomain, TryToDomain};
use crate::models::image_tag::ImageTagRow;
use crate::models::tag::{TagInsertRow, TagRow};
use crate::models::{SourceSiteName, TagType};
use kani_domain_api_model::tag::image_tag::{ImageTag, ImageTagId};
use kani_domain_api_model::tag::image_tag_source_site::ImageTagSourceSite;
use kani_domain_api_model::tag::{NewTag, Tag, TagId, TagName};

impl TryToDomain<Tag> for TagRow {
    type Error = anyhow::Error;

    fn try_to_domain(self) -> Result<Tag, Self::Error> {
        Ok(Tag {
            id: TagId(self.id),
            tag_type: (&self.tag_type).to_domain(),
            tag_name: TagName::try_from(self.tag_name)?,
        })
    }
}

impl ToDomain<kani_domain_api_model::tag::TagType> for &TagType {
    fn to_domain(self) -> kani_domain_api_model::tag::TagType {
        match self {
            TagType::RATING => kani_domain_api_model::tag::TagType::Rating,
            TagType::ARTIST => kani_domain_api_model::tag::TagType::Artist,
            TagType::SOURCE => kani_domain_api_model::tag::TagType::Source,
            TagType::CHARACTER => kani_domain_api_model::tag::TagType::Character,
            TagType::GENERAL => kani_domain_api_model::tag::TagType::General,
        }
    }
}

impl FromDomain<kani_domain_api_model::tag::TagType> for TagType {
    fn from_domain(value: kani_domain_api_model::tag::TagType) -> Self {
        match value {
            kani_domain_api_model::tag::TagType::Rating => TagType::RATING,
            kani_domain_api_model::tag::TagType::Artist => TagType::ARTIST,
            kani_domain_api_model::tag::TagType::Source => TagType::SOURCE,
            kani_domain_api_model::tag::TagType::Character => TagType::CHARACTER,
            kani_domain_api_model::tag::TagType::General => TagType::GENERAL,
        }
    }
}

impl TryToDomain<ImageTag> for (ImageTagRow, TagRow) {
    type Error = anyhow::Error;

    fn try_to_domain(self) -> Result<ImageTag, Self::Error> {
        let (image_tag_row, tag_row) = self;
        Ok(ImageTag {
            id: ImageTagId(image_tag_row.id),
            created_at: image_tag_row.created_at,
            tag: tag_row.try_to_domain()?,
            user_id: image_tag_row.user_id,
            source_site: image_tag_row.source_site.to_domain(),
        })
    }
}

impl ToDomain<ImageTagSourceSite> for SourceSiteName {
    fn to_domain(self) -> ImageTagSourceSite {
        match self {
            SourceSiteName::GELBOORU => ImageTagSourceSite::Gelbooru,
        }
    }
}

impl FromDomain<NewTag> for TagInsertRow {
    fn from_domain(value: NewTag) -> Self {
        Self {
            tag_type: FromDomain::from_domain(value.tag_type),
            tag_name: value.tag_name.into(),
        }
    }
}
