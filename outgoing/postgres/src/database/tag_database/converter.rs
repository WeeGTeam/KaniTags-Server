use crate::database::converter::{ToDomain, TryToDomain};
use crate::models::image_tag::ImageTagRow;
use crate::models::tag::TagRow;
use crate::models::{SourceSiteName, TagType};
use kani_domain_api_model::tag::image_tag::{ImageTag, ImageTagId};
use kani_domain_api_model::tag::tag_source_site::TagSourceSite;
use kani_domain_api_model::tag::{Tag, TagId, TagName};

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

impl ToDomain<TagSourceSite> for SourceSiteName {
    fn to_domain(self) -> TagSourceSite {
        match self {
            SourceSiteName::GELBOORU => TagSourceSite::Gelbooru,
        }
    }
}
