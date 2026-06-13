use crate::Postgres;
use kani_domain_api_outgoing::database::Database;

pub mod converter;
pub mod image_database;
pub mod similarity_database;
pub mod tag_database;
pub mod user_database;
pub mod import_session;
pub mod collection_database;

impl Database for Postgres {}
