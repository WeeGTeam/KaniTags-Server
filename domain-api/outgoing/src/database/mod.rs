pub use crate::database::image_database::ImageDatabase;
use crate::database::user_database::UserDatabase;

pub mod image_database;
pub mod user_database;

pub trait Database : ImageDatabase + UserDatabase {
}

