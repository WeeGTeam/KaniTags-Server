use crate::models::ReverseLookupSite;
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::auto_tag_session)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AutoTagSessionRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: i64,
    pub lookup_site: ReverseLookupSite,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = crate::schema::auto_tag_session)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AutoTagSessionInsertRow {
    pub user_id: i64,
    pub lookup_site: ReverseLookupSite,
    pub closed_at: Option<DateTime<Utc>>,
}
