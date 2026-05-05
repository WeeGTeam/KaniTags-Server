use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::collection_image)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CollectionImageRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub image_id: i64,
    pub collection_id: i64,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = crate::schema::collection_image)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CollectionImageInsertRow {
    pub image_id: i64,
    pub collection_id: i64,
}
