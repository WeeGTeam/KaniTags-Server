// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "auto_tag_status"))]
    pub struct AutoTagStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "image_format"))]
    pub struct ImageFormat;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "reverse_lookup_site"))]
    pub struct ReverseLookupSite;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "source_site_name"))]
    pub struct SourceSiteName;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "source_status"))]
    pub struct SourceStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tag_type"))]
    pub struct TagType;
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Bit;
    use super::sql_types::ReverseLookupSite;

    auto_tag_session (id) {
        id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        user_id -> Int8,
        lookup_site -> ReverseLookupSite,
        closed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Bit;
    use super::sql_types::AutoTagStatus;

    auto_tag_session_image (id) {
        id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        session_id -> Int8,
        image_id -> Int8,
        status -> AutoTagStatus,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Bit;
    use super::sql_types::SourceSiteName;

    auto_tag_session_image_result (id) {
        id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        session_image_id -> Int8,
        source_site -> SourceSiteName,
        #[max_length = 256]
        source_url -> Varchar,
        certainty -> Float8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Bit;

    collection (id) {
        id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        user_id -> Int8,
        #[max_length = 60]
        name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Bit;

    collection_image (id) {
        id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        image_id -> Int8,
        collection_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Bit;
    use super::sql_types::ImageFormat;

    image (id) {
        id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        id_hash -> Bytea,
        #[max_length = 144]
        perceptual_hash -> Bit,
        #[max_length = 60]
        file_name -> Varchar,
        image_format -> ImageFormat,
        res_width -> Int4,
        res_height -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Bit;
    use super::sql_types::ReverseLookupSite;
    use super::sql_types::SourceSiteName;
    use super::sql_types::SourceStatus;

    image_source (id) {
        id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        image_id -> Int8,
        reverse_lookup_site -> ReverseLookupSite,
        source_site -> SourceSiteName,
        source_status -> SourceStatus,
        #[max_length = 256]
        source_url -> Nullable<Varchar>,
        certainty -> Float8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Bit;
    use super::sql_types::SourceSiteName;

    image_tag (id) {
        id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        image_id -> Int8,
        tag_id -> Int8,
        user_id -> Nullable<Int8>,
        source_site -> Nullable<SourceSiteName>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Bit;

    import_session (id) {
        id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        user_id -> Int8,
        closed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Bit;

    import_session_image (id) {
        id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        import_id -> Int8,
        image_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Bit;
    use super::sql_types::TagType;

    tag (id) {
        id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        tag_type -> TagType,
        #[max_length = 40]
        tag_name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Bit;

    user_account (id) {
        id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 40]
        user_name -> Varchar,
        #[max_length = 40]
        display_name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::Bit;

    user_image (id) {
        id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        user_id -> Int8,
        image_id -> Int8,
    }
}

diesel::joinable!(auto_tag_session -> user_account (user_id));
diesel::joinable!(auto_tag_session_image -> auto_tag_session (session_id));
diesel::joinable!(auto_tag_session_image -> image (image_id));
diesel::joinable!(auto_tag_session_image_result -> auto_tag_session_image (session_image_id));
diesel::joinable!(collection -> user_account (user_id));
diesel::joinable!(collection_image -> collection (collection_id));
diesel::joinable!(collection_image -> image (image_id));
diesel::joinable!(image_source -> image (image_id));
diesel::joinable!(image_tag -> image (image_id));
diesel::joinable!(image_tag -> tag (tag_id));
diesel::joinable!(image_tag -> user_account (user_id));
diesel::joinable!(import_session -> user_account (user_id));
diesel::joinable!(import_session_image -> image (image_id));
diesel::joinable!(import_session_image -> import_session (import_id));
diesel::joinable!(user_image -> image (image_id));
diesel::joinable!(user_image -> user_account (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auto_tag_session,
    auto_tag_session_image,
    auto_tag_session_image_result,
    collection,
    collection_image,
    image,
    image_source,
    image_tag,
    import_session,
    import_session_image,
    tag,
    user_account,
    user_image,
);
