use crate::converter::{FromDomain, ToDomain};
use kani_domain_api_model::tag::{Tag, TagType};
use kani_openapi::models::{TagDto, TagTypeDto};

impl FromDomain<Tag> for TagDto {
    fn from_domain(tag: Tag) -> Self {
        Self {
            id: tag.id.to_string(),
            tag_type: FromDomain::from_domain(tag.tag_type),
            tag_name: (*tag.tag_name).to_owned(),
        }
    }
}

impl ToDomain<TagType> for &TagTypeDto {
    fn to_domain(self) -> TagType {
        match self {
            TagTypeDto::Rating => TagType::Rating,
            TagTypeDto::Artist => TagType::Artist,
            TagTypeDto::Source => TagType::Source,
            TagTypeDto::Character => TagType::Character,
            TagTypeDto::General => TagType::General,
        }
    }
}

impl FromDomain<TagType> for TagTypeDto {
    fn from_domain(tag_type: TagType) -> Self {
        match tag_type {
            TagType::Rating => TagTypeDto::Rating,
            TagType::Artist => TagTypeDto::Artist,
            TagType::Source => TagTypeDto::Source,
            TagType::Character => TagTypeDto::Character,
            TagType::General => TagTypeDto::General,
        }
    }
}
