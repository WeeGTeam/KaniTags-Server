use crate::models::ImageFormat;
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use pgvector::Bit;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::image)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ImageRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub id_hash: Vec<u8>,
    pub perceptual_hash: Bit,
    pub file_name: String,
    pub image_format: ImageFormat,
    pub res_width: i32,
    pub res_height: i32,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = crate::schema::image)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ImageInsertRow {
    pub id_hash: Vec<u8>,
    pub perceptual_hash: Bit,
    pub file_name: String,
    pub image_format: ImageFormat,
    pub res_width: i32,
    pub res_height: i32,
}
