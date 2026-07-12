use crate::models::TagType;
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::tag)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TagRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tag_type: TagType,
    pub tag_name: String,
}

#[derive(Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::tag)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TagInsertRow {
    pub tag_type: TagType,
    pub tag_name: String,
}
