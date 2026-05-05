use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::user_account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserAccountRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_name: String,
    pub display_name: String,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = crate::schema::user_account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserAccountInsertRow {
    pub user_name: String,
    pub display_name: String,
}
