pub use crate::api::outgoing::database::image_database::ImageDatabase;

pub mod image_database;

pub trait Database : ImageDatabase {
}

