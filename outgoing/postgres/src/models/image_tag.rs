use crate::models::SourceSiteName;
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::image_tag)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ImageTagRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub image_id: i64,
    pub tag_id: i64,
    pub user_id: Option<i64>,
    pub source_site: Option<SourceSiteName>,
}

#[derive(Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::image_tag)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ImageTagInsertRow {
    pub image_id: i64,
    pub tag_id: i64,
    pub user_id: Option<i64>,
    pub source_site: Option<SourceSiteName>,
}
