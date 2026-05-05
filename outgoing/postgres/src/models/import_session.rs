use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::import_session)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ImportSessionRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: i64,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = crate::schema::import_session)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ImportSessionInsertRow {
    pub user_id: i64,
    pub closed_at: Option<DateTime<Utc>>,
}
