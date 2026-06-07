use crate::dao::auto_tag_session::AutoTagDao;
use crate::dao::collection::CollectionDao;
use crate::dao::image::ImageDao;
use crate::dao::import_session::ImportSessionDao;
use crate::dao::tag::TagDao;
use crate::dao::user::UserDao;

pub mod auto_tag_session;
pub mod collection;
pub mod image;
pub mod import_session;
pub mod tag;
pub mod user;
pub mod image_search_query;

pub trait Dao {
    fn user_dao(&mut self) -> UserDao<'_>;
    fn image_dao(&mut self) -> ImageDao<'_>;
    fn import_session_dao(&mut self) -> ImportSessionDao<'_>;
    fn collection_dao(&mut self) -> CollectionDao<'_>;
    fn auto_tag_dao(&mut self) -> AutoTagDao<'_>;
    fn tag_dao(&mut self) -> TagDao<'_>;
}

impl Dao for diesel::PgConnection {
    fn user_dao(&mut self) -> UserDao<'_> {
        UserDao::new(self)
    }

    fn image_dao(&mut self) -> ImageDao<'_> {
        ImageDao::new(self)
    }

    fn import_session_dao(&mut self) -> ImportSessionDao<'_> {
        ImportSessionDao::new(self)
    }

    fn collection_dao(&mut self) -> CollectionDao<'_> {
        CollectionDao::new(self)
    }

    fn auto_tag_dao(&mut self) -> AutoTagDao<'_> {
        AutoTagDao::new(self)
    }

    fn tag_dao(&mut self) -> TagDao<'_> {
        TagDao::new(self)
    }
}

#[cfg(test)]
pub mod test {
    use crate::dao::Dao;
    use crate::models::auto_tag_session::{AutoTagSessionInsertRow, AutoTagSessionRow};
    use crate::models::auto_tag_session_image::{
        AutoTagSessionImageInsertRow, AutoTagSessionImageRow,
    };
    use crate::models::auto_tag_session_image_result::{
        AutoTagSessionImageResultInsertRow, AutoTagSessionImageResultRow,
    };
    use crate::models::collection::{CollectionInsertRow, CollectionRow};
    use crate::models::collection_image::{CollectionImageInsertRow, CollectionImageRow};
    use crate::models::image::{ImageInsertRow, ImageRow};
    use crate::models::image_source::{ImageSourceInsertRow, ImageSourceRow};
    use crate::models::image_tag::{ImageTagInsertRow, ImageTagRow};
    use crate::models::import_session::{ImportSessionInsertRow, ImportSessionRow};
    use crate::models::import_session_image::{ImportSessionImageInsertRow, ImportSessionImageRow};
    use crate::models::tag::{TagInsertRow, TagRow};
    use crate::models::user_account::{UserAccountInsertRow, UserAccountRow};
    use crate::models::user_image::{UserImageInsertRow, UserImageRow};
    use crate::models::{
        AutoTagStatus, ImageFormat, ReverseLookupSite, SourceSiteName, SourceStatus, TagType,
    };
    use anyhow::Error;
    use diesel::PgConnection;
    use pgvector::Bit;
    use rand::random;

    pub fn insert_test_user(c: &mut PgConnection) -> Result<UserAccountRow, Error> {
        c.user_dao().insert_user(&UserAccountInsertRow {
            user_name: format!("test_user{}", random::<u32>()).to_string(),
            display_name: "Test User".to_string(),
        })
    }

    pub fn insert_test_image(c: &mut PgConnection) -> Result<ImageRow, Error> {
        c.image_dao().insert_image(&ImageInsertRow {
            id_hash: vec![0, 1, 2, 3, 4, random::<u8>(), random::<u8>(), random::<u8>()],
            perceptual_hash: Bit::from_bytes(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]),
            file_name: "test_image.jpg".to_string(),
            image_format: ImageFormat::JPG,
            res_width: 1920,
            res_height: 1080,
        })
    }

    pub fn insert_test_user_image(
        c: &mut PgConnection,
        user_id: i64,
        image_id: i64,
    ) -> Result<UserImageRow, Error> {
        c.image_dao()
            .insert_user_image(&UserImageInsertRow { user_id, image_id })
    }

    pub fn insert_test_image_source(
        c: &mut PgConnection,
        image_id: i64,
    ) -> Result<ImageSourceRow, Error> {
        c.image_dao().insert_image_source(&ImageSourceInsertRow {
            image_id,
            reverse_lookup_site: ReverseLookupSite::IQDB,
            source_site: SourceSiteName::GELBOORU,
            source_status: SourceStatus::EXISTING,
            source_url: Some("example.com".to_string()),
            certainty: 0.76,
        })
    }

    pub fn insert_test_import_session(
        c: &mut PgConnection,
        user_id: i64,
    ) -> Result<ImportSessionRow, Error> {
        c.import_session_dao()
            .insert_import_session(&ImportSessionInsertRow { user_id })
    }

    pub fn insert_test_import_session_image(
        c: &mut PgConnection,
        import_session_id: i64,
        image_id: i64,
    ) -> Result<ImportSessionImageRow, Error> {
        c.import_session_dao()
            .insert_import_session_images(&[ImportSessionImageInsertRow {
                import_id: import_session_id,
                image_id,
            }])
            .map(|v| v.into_iter().next().unwrap())
    }

    pub fn insert_test_tag(c: &mut PgConnection) -> Result<TagRow, Error> {
        c.tag_dao().insert_tag(&TagInsertRow {
            tag_type: TagType::CHARACTER,
            tag_name: format!("Megumin-{}", random::<u8>()),
        })
    }

    pub fn insert_test_image_tag(
        c: &mut PgConnection,
        image_id: i64,
        tag_id: i64,
        user_id: Option<i64>,
    ) -> Result<ImageTagRow, Error> {
        c.tag_dao().insert_image_tag(&ImageTagInsertRow {
            image_id,
            tag_id,
            user_id,
            source_site: None,
        })
    }

    pub fn insert_test_collection(
        c: &mut PgConnection,
        user_id: i64,
    ) -> Result<CollectionRow, Error> {
        c.collection_dao().insert_collection(&CollectionInsertRow {
            user_id,
            name: format!("test_collection {}", random::<u32>()),
        })
    }

    pub fn insert_test_collection_image(
        c: &mut PgConnection,
        collection_id: i64,
        image_id: i64,
    ) -> Result<CollectionImageRow, Error> {
        c.collection_dao()
            .insert_collection_images(&[CollectionImageInsertRow {
                image_id,
                collection_id,
            }])
            .map(|v| v.into_iter().next().unwrap())
    }

    pub fn insert_test_auto_tag_session(
        c: &mut PgConnection,
        user_id: i64,
    ) -> Result<AutoTagSessionRow, Error> {
        c.auto_tag_dao()
            .insert_auto_tag_session(&AutoTagSessionInsertRow {
                user_id,
                lookup_site: ReverseLookupSite::IQDB,
                closed_at: None,
            })
    }

    pub fn insert_test_auto_tag_session_image(
        c: &mut PgConnection,
        session_id: i64,
        image_id: i64,
    ) -> Result<AutoTagSessionImageRow, Error> {
        c.auto_tag_dao()
            .insert_auto_tag_session_images(&[AutoTagSessionImageInsertRow {
                session_id,
                image_id,
                status: AutoTagStatus::PENDING,
            }])
            .map(|v| v.into_iter().next().unwrap())
    }

    pub fn insert_test_auto_tag_session_image_results(
        c: &mut PgConnection,
        session_image_id: i64,
    ) -> Result<Vec<AutoTagSessionImageResultRow>, Error> {
        c.auto_tag_dao().insert_auto_tag_session_image_results(&[
            AutoTagSessionImageResultInsertRow {
                session_image_id,
                source_site: SourceSiteName::GELBOORU,
                source_url: "example.com".to_string(),
                certainty: 0.85,
            },
        ])
    }
}
