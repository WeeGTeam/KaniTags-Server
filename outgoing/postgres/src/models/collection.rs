use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::collection)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CollectionRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: i64,
    pub name: String,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = crate::schema::collection)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CollectionInsertRow {
    pub user_id: i64,
    pub name: String,
}
