use crate::database::converter::{ToDomain, TryToDomain};
use crate::models::tag::TagRow;
use crate::models::TagType;
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
