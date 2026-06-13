use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct CollectionId(pub i64);

#[derive(Debug, Clone)]
pub struct Collection {
    pub id: i64,
    pub name: String,
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

impl std::ops::DerefMut for CollectionId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
