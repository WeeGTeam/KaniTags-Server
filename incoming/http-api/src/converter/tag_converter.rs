use crate::converter::{FromDomain, ToDomain};
use kani_domain_api_model::tag::{Tag, TagType};

impl FromDomain<Tag> for kani_openapi::models::Tag {
    fn from_domain(tag: Tag) -> Self {
        Self {
            id: tag.id.to_string(),
            tag_type: FromDomain::from_domain(tag.tag_type),
            tag_name: tag.tag_name.to_owned(),
        }
    }
}

impl ToDomain<TagType> for &kani_openapi::models::TagType {
    fn to_domain(self) -> TagType {
        match self {
            kani_openapi::models::TagType::Rating => TagType::Rating,
            kani_openapi::models::TagType::Artist => TagType::Artist,
            kani_openapi::models::TagType::Source => TagType::Source,
            kani_openapi::models::TagType::Character => TagType::Character,
            kani_openapi::models::TagType::General => TagType::General,
        }
    }
}

impl FromDomain<TagType> for kani_openapi::models::TagType {
    fn from_domain(tag_type: TagType) -> Self {
        match tag_type {
            TagType::Rating => kani_openapi::models::TagType::Rating,
            TagType::Artist => kani_openapi::models::TagType::Artist,
            TagType::Source => kani_openapi::models::TagType::Source,
            TagType::Character => kani_openapi::models::TagType::Character,
            TagType::General => kani_openapi::models::TagType::General,
        }
    }
}
