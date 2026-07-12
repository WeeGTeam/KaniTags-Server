use anyhow::anyhow;
use std::ops::Deref;

pub mod image_tag;
pub mod tag_source_site;

#[derive(Debug, Clone)]
pub struct NewTag {
    pub tag_type: TagType,
    pub tag_name: TagName,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    pub id: TagId,
    pub tag_type: TagType,
    pub tag_name: TagName,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TagId (pub i64);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TagType {
    Rating,
    Artist,
    Source,
    Character,
    General,
}

#[derive(Debug, Clone,  PartialEq, Eq, PartialOrd, Ord)]
pub struct TagName (String);

impl Deref for TagId {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<String> for TagName {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 40 {
            Ok(Self(value))
        } else {
            Err(anyhow!("Tag name must not be longer than 40 characters: \"{}\"", value))
        }
    }
}

impl From<TagName> for String {
    fn from(value: TagName) -> Self {
        value.0
    }
}

impl Deref for TagName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "test-util")]
pub mod stub {
    use super::*;

    impl NewTag {
        pub fn stub() -> Self {
            NewTag {
                tag_type: TagType::Character,
                tag_name: TagName::try_from("Megumin".to_owned()).unwrap(),
            }
        }
    }
}
