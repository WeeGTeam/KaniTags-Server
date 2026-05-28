use crate::models::ImageFormat;
use chrono::{DateTime, Utc};
use diesel::sql_types::BigInt;
use diesel::sql_types::Integer;
use diesel::{Insertable, Queryable, QueryableByName, Selectable};
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

#[derive(QueryableByName, Debug)]
pub struct SimilarImagePairRow {
    #[diesel(sql_type = BigInt)]
    pub id1: i64,
    #[diesel(sql_type = BigInt)]
    pub id2: i64,
    #[diesel(sql_type = Integer)]
    pub dist: i32,
}
