use anyhow::anyhow;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct CollectionId(pub i64);

#[derive(Debug, Clone, PartialEq)]
pub struct CollectionName(String);

#[derive(Debug, Clone)]
pub struct Collection {
    pub id: CollectionId,
    pub name: CollectionName,
    pub created_by: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl std::ops::Deref for CollectionId {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Deref for CollectionName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<String> for CollectionName {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 60 {
            Ok(Self(value))
        } else {
            Err(anyhow!("collection name must not be longer than 60 characters: '{}'", value))
        }
    }
}
