use crate::Postgres;
use kani_domain_api_outgoing::database::collection_database::CollectionDatabase;
use kani_domain_api_outgoing::database::import_session::ImportSessionDatabase;
use kani_domain_api_outgoing::database::similarity_database::SimilarityDatabase;
use kani_domain_api_outgoing::database::tag_database::TagDatabase;
use kani_domain_api_outgoing::database::user_database::UserDatabase;
use kani_domain_api_outgoing::database::{Database, ImageDatabase};

pub mod converter;
pub mod image_database;
pub mod similarity_database;
pub mod tag_database;
pub mod user_database;
pub mod import_session;
pub mod collection_database;

impl Database for Postgres {
    fn image(&self) -> &dyn ImageDatabase {
        self
    }
    fn user(&self) -> &dyn UserDatabase {
        self
    }

    fn similarity(&self) -> &dyn SimilarityDatabase {
        self
    }

    fn tag(&self) -> &dyn TagDatabase {
        self
    }

    fn import_session(&self) -> &dyn ImportSessionDatabase {
        self
    }

    fn collection(&self) -> &dyn CollectionDatabase {
        self
    }
}
