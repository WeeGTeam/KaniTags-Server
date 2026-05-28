pub use crate::database::image_database::ImageDatabase;
use crate::database::similarity_database::SimilarityDatabase;
use crate::database::user_database::UserDatabase;

pub mod image_database;
pub mod similarity_database;
pub mod user_database;

pub trait Database : ImageDatabase + UserDatabase + SimilarityDatabase {
}

