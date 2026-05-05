use crate::models::AutoTagStatus;
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::auto_tag_session_image)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AutoTagSessionImageRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub session_id: i64,
    pub image_id: i64,
    pub status: AutoTagStatus,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = crate::schema::auto_tag_session_image)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AutoTagSessionImageInsertRow {
    pub session_id: i64,
    pub image_id: i64,
    pub status: AutoTagStatus,
}
