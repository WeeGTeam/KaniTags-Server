pub mod auto_tag_session;
pub mod auto_tag_session_image;
pub mod auto_tag_session_image_result;
pub mod collection;
pub mod collection_image;
pub mod image;
pub mod image_source;
pub mod image_tag;
pub mod import_session;
pub mod import_session_image;
pub mod tag;
pub mod user_account;
pub mod user_image;

#[derive(Clone, Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::AutoTagStatus"]
#[DbValueStyle = "UPPERCASE"]
pub enum AutoTagStatus {
    PENDING,
    COMPLETE,
}

#[derive(Clone, Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::ImageFormat"]
#[DbValueStyle = "UPPERCASE"]
pub enum ImageFormat {
    PNG,
    JPG,
}

#[derive(Clone, Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::ReverseLookupSite"]
#[DbValueStyle = "UPPERCASE"]
pub enum ReverseLookupSite {
    IQDB,
}

#[derive(Clone, Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::SourceSiteName"]
#[DbValueStyle = "UPPERCASE"]
pub enum SourceSiteName {
    GELBOORU,
}

#[derive(Clone, Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::SourceStatus"]
#[DbValueStyle = "UPPERCASE"]
pub enum SourceStatus {
    EXISTING,
    MISSING,
}

#[derive(Clone, Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::TagType"]
#[DbValueStyle = "UPPERCASE"]
pub enum TagType {
    RATING,
    ARTIST,
    SOURCE,
    CHARACTER,
    GENERAL,
}
