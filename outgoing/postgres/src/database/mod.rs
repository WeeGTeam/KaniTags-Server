use crate::Postgres;
use kani_domain_api_outgoing::database::Database;

pub mod image_database;
pub mod similarity_database;
pub mod user_database;

impl Database for Postgres {}
