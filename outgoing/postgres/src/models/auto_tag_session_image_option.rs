use crate::models::SourceSiteName;
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::auto_tag_session_image_option)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AutoTagSessionImageOptionRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub session_image_id: i64,
    pub source_site: SourceSiteName,
    pub source_url: String,
    pub certainty: f64,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = crate::schema::auto_tag_session_image_option)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AutoTagSessionImageOptionInsertRow {
    pub session_image_id: i64,
    pub source_site: SourceSiteName,
    pub source_url: String,
    pub certainty: f64,
}
