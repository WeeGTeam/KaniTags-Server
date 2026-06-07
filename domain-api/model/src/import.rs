use chrono::{DateTime, Utc};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct ImportSessionId(pub i64);

pub struct ImportSession {
    pub id: i64,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

impl Deref for ImportSessionId {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
