use crate::models::image::{ImageInsertRow, ImageRow};
use crate::models::image_source::{ImageSourceInsertRow, ImageSourceRow};
use crate::models::user_image::{UserImageInsertRow, UserImageRow};
use crate::schema::image::dsl as image_dsl;
use crate::schema::image::dsl::image;
use crate::schema::image_source::dsl as image_source_dsl;
use crate::schema::image_source::dsl::image_source;
use crate::schema::user_image::dsl as user_image_dsl;
use crate::schema::user_image::dsl::user_image;
use anyhow::Context;
use anyhow::Error;
use diesel::{ExpressionMethods, OptionalExtension};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

pub struct ImageDao<'c> {
    connection: &'c mut diesel::PgConnection,
}

impl<'c> ImageDao<'c> {
    pub fn new(connection: &'c mut diesel::PgConnection) -> Self {
        Self { connection }
    }

    pub fn insert_image(
        &mut self,
        insert_row: &ImageInsertRow,
    ) -> Result<ImageRow, Error> {
        diesel::insert_into(image)
            .values(insert_row)
            .returning(ImageRow::as_returning())
            .get_result(self.connection)
            .context("Failed to insert image into database")
    }

    pub fn get_all_images(&mut self) -> Result<Vec<ImageRow>, Error> {
        image.load(self.connection)
            .context("Failed to get all images from database")
    }

    pub fn insert_user_image(
        &mut self,
        insert_row: &UserImageInsertRow,
    ) -> Result<UserImageRow, Error> {
        diesel::insert_into(user_image)
            .values(insert_row)
            .returning(UserImageRow::as_returning())
            .get_result(self.connection)
            .context("Failed to insert user image into database")
    }

    pub fn get_all_images_by_user(
        &mut self,
        user_id: i64,
    ) -> Result<Vec<ImageRow>, Error> {
        image
            .select(ImageRow::as_select())
            .inner_join(user_image)
            .filter(user_image_dsl::id.eq(user_id))
            .load(self.connection)
            .context("Failed to get images by user from database")
    }

    pub fn get_image_by_id(&mut self, id: i64) -> Result<ImageRow, Error> {
        image
            .filter(image_dsl::id.eq(id))
            .get_result(self.connection)
            .context("Failed to get image by id from database")
    }

    pub fn get_image_by_id_hash(
        &mut self,
        id_hash: &[u8],
    ) -> Result<ImageRow, Error> {
        image
            .filter(image_dsl::id_hash.eq(id_hash))
            .get_result(self.connection)
            .context("Failed to get image by id hash from database")
    }

    pub fn insert_image_source(
        &mut self,
        insert_row: &ImageSourceInsertRow,
    ) -> Result<ImageSourceRow, Error> {
        diesel::insert_into(image_source)
            .values(insert_row)
            .returning(ImageSourceRow::as_returning())
            .get_result(self.connection)
            .context("Failed to insert image source into database")
    }

    pub fn get_image_sources_by_image(
        &mut self,
        image_id: i64,
    ) -> Result<Vec<ImageSourceRow>, Error> {
        image_source
            .filter(image_source_dsl::image_id.eq(image_id))
            .load(self.connection)
            .context("Failed to get image sources by image from database")
    }
}

#[cfg(test)]
mod test {
    use crate::dao::test::{insert_test_image, insert_test_image_source, insert_test_user};
    use crate::dao::Dao;
    use crate::models::image::ImageInsertRow;
    use crate::models::image_source::ImageSourceInsertRow;
    use crate::models::user_image::UserImageInsertRow;
    use crate::models::{ImageFormat, ReverseLookupSite, SourceSiteName, SourceStatus};
    use crate::test::test_db;
    use diesel::Connection;

    #[test]
    #[serial_test::serial]
    fn test_insert_image() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            c.image_dao().insert_image(&ImageInsertRow {
                id_hash: vec![0, 1, 2, 3, 4, 5, 6, 7],
                perceptual_hash: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
                file_name: "test_image.jpg".to_string(),
                image_format: ImageFormat::JPG,
                res_width: 1920,
                res_height: 1080,
            })
        });
        println!("image: {:?}", result);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_image_by_user_id() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let _image = insert_test_image(c)?;
            c.image_dao().get_all_images()
        });
        for result in results {
            println!("image: {:?}", result);
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_insert_user_image() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let image = insert_test_image(c)?;
            c.image_dao().insert_user_image(&UserImageInsertRow {
                user_id: user.id,
                image_id: image.id,
            })
        });
        println!("user image: {:?}", result);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_all_images_by_user() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            c.image_dao().get_all_images_by_user(user.id)
        });
        println!("image: {:?}", result);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_image_by_id() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let image = insert_test_image(c)?;
            c.image_dao().get_image_by_id(image.id)
        });
        println!("image: {:?}", result);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_image_by_id_hash() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let image = insert_test_image(c)?;
            c.image_dao().get_image_by_id_hash(&image.id_hash)
        });
        println!("image: {:?}", result);
    }

    #[test]
    #[serial_test::serial]
    fn test_insert_image_source() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let image = insert_test_image(c)?;
            c.image_dao().insert_image_source(&ImageSourceInsertRow {
                image_id: image.id,
                reverse_lookup_site: ReverseLookupSite::IQDB,
                source_site: SourceSiteName::GELBOORU,
                source_status: SourceStatus::EXISTING,
                source_url: Some("example.com".to_string()),
                certainty: 0.77,
            })
        });
        println!("image source: {:?}", result);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_image_source_by_image() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let image = insert_test_image(c)?;
            let _source = insert_test_image_source(c, image.id)?;
            c.image_dao().get_image_sources_by_image(image.id)
        });
        for result in results {
            println!("image source: {:?}", result);
        }
    }
}
