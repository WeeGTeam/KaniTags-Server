pub use crate::database::image_database::ImageDatabase;
use crate::database::import_session::ImportSessionDatabase;
use crate::database::similarity_database::SimilarityDatabase;
use crate::database::tag_database::TagDatabase;
use crate::database::user_database::UserDatabase;

pub mod image_database;
pub mod import_session;
pub mod similarity_database;
pub mod tag_database;
pub mod user_database;

pub trait Database : ImageDatabase + UserDatabase + SimilarityDatabase + TagDatabase + ImportSessionDatabase {
}

