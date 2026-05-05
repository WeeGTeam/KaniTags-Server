use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::user_image)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserImageRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: i64,
    pub image_id: i64,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = crate::schema::user_image)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserImageInsertRow {
    pub user_id: i64,
    pub image_id: i64,
}
