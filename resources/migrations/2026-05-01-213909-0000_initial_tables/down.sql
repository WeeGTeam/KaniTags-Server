-- This file should undo anything in `up.sql`

DROP TABLE auto_tag_session_image_result;
DROP TABLE auto_tag_session_image;
DROP TABLE auto_tag_session;
DROP TABLE import_session_image;
DROP TABLE import_session;
DROP TABLE collection_image;
DROP TABLE collection;
DROP TABLE image_tag;
DROP TABLE tag;
DROP TABLE user_image;
DROP TABLE image_source;
DROP TABLE image;
DROP TABLE user_account;

DROP TYPE image_format;
DROP TYPE reverse_lookup_site;
DROP TYPE source_site_name;
DROP TYPE source_status;
DROP TYPE tag_type;
DROP TYPE auto_tag_status;
