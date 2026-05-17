pub use crate::database::image_database::ImageDatabase;

pub mod image_database;

pub trait Database : ImageDatabase {
}

