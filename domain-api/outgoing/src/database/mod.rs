use crate::database::collection_database::CollectionDatabase;
pub use crate::database::image_database::ImageDatabase;
use crate::database::import_session::ImportSessionDatabase;
use crate::database::similarity_database::SimilarityDatabase;
use crate::database::tag_database::TagDatabase;
use crate::database::user_database::UserDatabase;

pub mod collection_database;
pub mod image_database;
pub mod import_session;
pub mod similarity_database;
pub mod tag_database;
pub mod user_database;

pub trait Database {
    fn image(&self) -> &dyn ImageDatabase;
    fn user(&self) -> &dyn UserDatabase;
    fn similarity(&self) -> &dyn SimilarityDatabase;
    fn tag(&self) -> &dyn TagDatabase;
    fn import_session(&self) -> &dyn ImportSessionDatabase;
    fn collection(&self) -> &dyn CollectionDatabase;
}


#[cfg(feature = "test-util")]
pub mod mock {
    use super::*;
    use crate::database::collection_database::mock::MockCollectionDatabase;
    use crate::database::image_database::mock::MockImageDatabase;
    use crate::database::import_session::mock::MockImportSessionDatabase;
    use crate::database::similarity_database::mock::MockSimilarityDatabase;
    use crate::database::tag_database::mock::MockTagDatabase;
    use crate::database::user_database::mock::MockUserDatabase;

    pub struct MockDatabase {
        mock_image_database: MockImageDatabase,
        mock_user_database: MockUserDatabase,
        mock_similarity_database: MockSimilarityDatabase,
        mock_tag_database: MockTagDatabase,
        mock_import_session_database: MockImportSessionDatabase,
        mock_collection_database: MockCollectionDatabase,
    }

    impl Default for MockDatabase {
        fn default() -> Self {
            Self {
                mock_image_database: MockImageDatabase::default(),
                mock_user_database: MockUserDatabase::default(),
                mock_similarity_database: MockSimilarityDatabase::default(),
                mock_tag_database: MockTagDatabase::default(),
                mock_import_session_database: MockImportSessionDatabase::default(),
                mock_collection_database: MockCollectionDatabase::default(),
            }
        }
    }

    impl MockDatabase {
        pub fn with_image_database(mut self, mock_image_database: MockImageDatabase) -> Self {
            self.mock_image_database = mock_image_database;
            self
        }

        pub fn with_user_database(mut self, mock_user_database: MockUserDatabase) -> Self {
            self.mock_user_database = mock_user_database;
            self
        }

        pub fn with_similarity_database(mut self, mock_similarity_database: MockSimilarityDatabase) -> Self {
            self.mock_similarity_database = mock_similarity_database;
            self
        }

        pub fn with_tag_database(mut self, mock_tag_database: MockTagDatabase) -> Self {
            self.mock_tag_database = mock_tag_database;
            self
        }

        pub fn with_import_session_database(mut self, mock_import_session_database: MockImportSessionDatabase) -> Self {
            self.mock_import_session_database = mock_import_session_database;
            self
        }

        pub fn with_collection_database(mut self, mock_collection_database: MockCollectionDatabase) -> Self {
            self.mock_collection_database = mock_collection_database;
            self
        }
    }

    impl Database for MockDatabase {
        fn image(&self) -> &dyn ImageDatabase {
            &self.mock_image_database
        }

        fn user(&self) -> &dyn UserDatabase {
            &self.mock_user_database
        }

        fn similarity(&self) -> &dyn SimilarityDatabase {
            &self.mock_similarity_database
        }

        fn tag(&self) -> &dyn TagDatabase {
            &self.mock_tag_database
        }

        fn import_session(&self) -> &dyn ImportSessionDatabase {
            &self.mock_import_session_database
        }

        fn collection(&self) -> &dyn CollectionDatabase {
            &self.mock_collection_database
        }
    }
}
