use crate::image_id::ImageId;
use crate::tag::tag_source_site::TagSourceSite;
use crate::tag::{Tag, TagId};
use crate::user::User;
use chrono::{DateTime, Utc};
use std::ops::Deref;

pub struct NewImageTag {
    pub image_id: ImageId,
    pub tag_id: TagId,
    pub user: Option<User>,
    pub source_site: Option<TagSourceSite>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageTag {
    pub id: ImageTagId,
    pub created_at: DateTime<Utc>,
    pub tag: Tag,
    pub user_id: Option<i64>,
    pub source_site: Option<TagSourceSite>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageTagId(pub i64);

impl Deref for ImageTagId {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "test-util")]
pub mod stub {
    use super::*;

    impl ImageTag {
        pub fn stub() -> Self {
            ImageTag {
                id: ImageTagId(i64::MAX),
                created_at: DateTime::<Utc>::MIN_UTC,
                tag: Tag::stub(),
                user_id: Some(i64::MAX),
                source_site: None,
            }
        }
    }
}
