use crate::models::{ReverseLookupSite, SourceSiteName, SourceStatus};
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::image_source)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ImageSourceRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub image_id: i64,
    pub reverse_lookup_site: ReverseLookupSite,
    pub source_site: SourceSiteName,
    pub source_status: SourceStatus,
    pub source_url: Option<String>,
    pub certainty: f64,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = crate::schema::image_source)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ImageSourceInsertRow {
    pub image_id: i64,
    pub reverse_lookup_site: ReverseLookupSite,
    pub source_site: SourceSiteName,
    pub source_status: SourceStatus,
    pub source_url: Option<String>,
    pub certainty: f64,
}
