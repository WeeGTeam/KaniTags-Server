use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::import_session_image)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ImportSessionImageRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub import_id: i64,
    pub image_id: i64,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = crate::schema::import_session_image)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ImportSessionImageInsertRow {
    pub import_id: i64,
    pub image_id: i64,
}
