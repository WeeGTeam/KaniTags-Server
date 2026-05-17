use crate::Postgres;
use kani_domain_api_outgoing::database::Database;

pub mod image_database;

impl Database for Postgres {}
