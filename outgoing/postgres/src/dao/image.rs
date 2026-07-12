use crate::dao::image_search_query::ImageSearchQueryBuilder;
use crate::models::image::{ImageInsertRow, ImageRow, SimilarImagePairRow};
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
use diesel::sql_types::{BigInt, Bytea, Integer};
use diesel::{BoolExpressionMethods, ExpressionMethods, OptionalExtension};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use kani_domain_api_model::image_search::ImageSearchFilter;

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
            .filter(user_image_dsl::user_id.eq(user_id))
            .load(self.connection)
            .context("Failed to get images by user from database")
    }

    pub fn search_images(&mut self, user_id: i64, filter: &ImageSearchFilter) -> Result<Vec<ImageRow>, Error> {
        ImageSearchQueryBuilder::for_user(user_id)
            .with_dimensions(filter)
            .with_layout(filter.layout.as_ref())
            .with_tags(&filter.tags)
            .excluding_tags(&filter.exclude_tags)
            .in_import_session(filter.import_session.as_ref())
            .in_collection(filter.collection.as_ref())
            .sorted_by(&filter.sort)
            .load(self.connection)
            .context("Failed to search images by user and filter from database")
    }

    pub fn get_image_by_id(&mut self, id: i64) -> Result<Option<ImageRow>, Error> {
        image
            .select(ImageRow::as_select())
            .filter(image_dsl::id.eq(id))
            .get_result(self.connection)
            .optional()
            .context("Failed to get image by id from database")
    }

    pub fn get_image_by_id_hash(
        &mut self,
        id_hash: &[u8],
    ) -> Result<Option<ImageRow>, Error> {
        image
            .filter(image_dsl::id_hash.eq(id_hash))
            .get_result(self.connection)
            .optional()
            .context("Failed to get image by id hash from database")
    }

    pub fn get_images_by_user_and_ids(
        &mut self,
        user_id: i64,
        image_ids: &[i64],
    ) -> Result<Vec<ImageRow>, Error> {
        image
            .select(ImageRow::as_select())
            .inner_join(user_image)
            .filter(image_dsl::id.eq_any(image_ids).and(user_image_dsl::user_id.eq(user_id)))
            .get_results(self.connection)
            .context("Failed to get images by user and id hashes from database")
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

    pub fn get_similar_images_by_id_hash(&mut self, id_hash: &[u8], max_distance: i32, neighbors_per_row: i64) -> Result<Vec<SimilarImagePairRow>, Error> {
        diesel::sql_query(
            r#"
            SELECT i1.id_hash AS id_hash1,
                   i2.id_hash AS id_hash2,
                   (i1.perceptual_hash <~> i2.perceptual_hash)::int AS dist
            FROM image i1
                     CROSS JOIN LATERAL (
                SELECT id, id_hash, perceptual_hash
                FROM image
                ORDER BY perceptual_hash <~> i1.perceptual_hash
                LIMIT $1
                ) i2
            WHERE i1.id_hash = $2 AND i1.id <> i2.id
              AND i1.perceptual_hash <~> i2.perceptual_hash < $3
            ORDER BY dist;
            "#,
        )
            .bind::<BigInt, _>(neighbors_per_row)
            .bind::<Bytea, _>(id_hash)
            .bind::<Integer, _>(max_distance)
            .load::<SimilarImagePairRow>(self.connection)
            .context("Failed to get similar images from database")
    }

    pub fn get_all_similar_images(&mut self, max_distance: i32, neighbors_per_row: i64) -> Result<Vec<SimilarImagePairRow>, Error> {
        diesel::sql_query(
            r#"
            SELECT i1.id_hash AS id_hash1,
                   i2.id_hash AS id_hash2,
                   (i1.perceptual_hash <~> i2.perceptual_hash)::int AS dist
            FROM image i1
                     CROSS JOIN LATERAL (
                SELECT id, id_hash, perceptual_hash
                FROM image
                ORDER BY perceptual_hash <~> i1.perceptual_hash
                LIMIT $1
                ) i2
            WHERE i1.id <> i2.id
              AND i1.perceptual_hash <~> i2.perceptual_hash < $2;
            "#,
        )
            .bind::<BigInt, _>(neighbors_per_row)
            .bind::<Integer, _>(max_distance)
            .load::<SimilarImagePairRow>(self.connection)
            .context("Failed to get all similar images from database")
    }

}

#[cfg(test)]
mod test {
    use crate::dao::test::{insert_test_image, insert_test_image_source, insert_test_image_tag, insert_test_tag, insert_test_user, insert_test_user_image};
    use crate::dao::Dao;
    use crate::models::image::ImageInsertRow;
    use crate::models::image_source::ImageSourceInsertRow;
    use crate::models::user_image::UserImageInsertRow;
    use crate::models::{ImageFormat, ReverseLookupSite, SourceSiteName, SourceStatus};
    use crate::test::test_db;
    use assertables::{assert_len_eq_x, assert_matches, assert_some};
    use diesel::Connection;
    use kani_domain_api_model::image_search::ImageSearchFilter;
    use kani_domain_api_model::tag::TagId;
    use pgvector::Bit;

    #[test]
    #[serial_test::serial]
    fn test_insert_image() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let _result = conn.test_transaction(|c| {
            c.image_dao().insert_image(&ImageInsertRow {
                id_hash: vec![0, 1, 2, 3, 4, 5, 6, 7],
                perceptual_hash: Bit::from_bytes(&vec![
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                ]),
                file_name: "test_image.jpg".to_string(),
                image_format: ImageFormat::JPG,
                res_width: 1920,
                res_height: 1080,
            })
        });
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
        assert_len_eq_x!(results, 1);
    }

    #[test]
    #[serial_test::serial]
    fn test_insert_user_image() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let _result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let image = insert_test_image(c)?;
            c.image_dao().insert_user_image(&UserImageInsertRow {
                user_id: user.id,
                image_id: image.id,
            })
        });
    }

    #[test]
    #[serial_test::serial]
    fn test_get_all_images_by_user() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let image = insert_test_image(c)?;
            let _user_image = insert_test_user_image(c, user.id, image.id)?;
            c.image_dao().get_all_images_by_user(user.id)
        });
        assert_len_eq_x!(result, 1);
    }

    #[test]
    #[serial_test::serial]
    fn test_search_images() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let image = insert_test_image(c)?;
            let image2 = insert_test_image(c)?;
            let image3 = insert_test_image(c)?;
            let _user_image = insert_test_user_image(c, user.id, image.id)?;
            let _user_image2 = insert_test_user_image(c, user.id, image2.id)?;
            let _user_image3 = insert_test_user_image(c, user.id, image3.id)?;
            let tag = insert_test_tag(c)?;
            let etag = insert_test_tag(c)?;
            let _image_tag = insert_test_image_tag(c, image.id, tag.id, Some(user.id))?;
            let _image_tag2 = insert_test_image_tag(c, image2.id, tag.id, Some(user.id))?;
            let _image_etag2 = insert_test_image_tag(c, image2.id, etag.id, Some(user.id))?;
            let mut filter = ImageSearchFilter::default();
            filter.tags.push(TagId(tag.id));
            filter.exclude_tags.push(TagId(etag.id));
            c.image_dao().search_images(user.id, &filter)
        });
        assert_len_eq_x!(result, 1);
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
        assert_some!(result);
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
        assert_some!(result);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_images_by_user_and_ids() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let image = insert_test_image(c)?;
            let image2 = insert_test_image(c)?;
            let image3 = insert_test_image(c)?;
            let _user_image = insert_test_user_image(c, user.id, image.id)?;
            let _user_image2 = insert_test_user_image(c, user.id, image2.id)?;
            c.image_dao().get_images_by_user_and_ids(user.id, &[image.id, image2.id, image3.id])
        });
        assert_len_eq_x!(result, 2);
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
        assert_matches!(result.reverse_lookup_site, ReverseLookupSite::IQDB);
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
        assert_len_eq_x!(results, 1);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_similar_images() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let image = insert_test_image(c)?;
            let _image2 = insert_test_image(c)?;
            c.image_dao().get_similar_images_by_id_hash(&image.id_hash, 30, 40)
        });
        assert_len_eq_x!(results, 1);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_all_similar_images() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let _image = insert_test_image(c)?;
            let _image2 = insert_test_image(c)?;
            c.image_dao().get_all_similar_images(30, 40)
        });
        assert_len_eq_x!(results, 2);
    }
}

#[cfg(test)]
mod test_data {
    use crate::models::image::ImageInsertRow;
    use crate::models::ImageFormat;
    use crate::test::test_db;
    use diesel::{PgConnection, RunQueryDsl};
    use pgvector::Bit;
    use rand::RngExt;

    fn random_byte_array<const N: usize>() -> [u8; N] {
        let mut rng = rand::rng();
        (0..N)
            .map(|_| rng.random::<u8>())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn insert_test_image(c: &mut PgConnection, rows: &[ImageInsertRow]) {
        diesel::insert_into(crate::schema::image::table)
            .values(rows)
            .execute(c)
            .unwrap();
    }

    fn create_insert_row(index: usize) -> ImageInsertRow {
        ImageInsertRow {
            id_hash: random_byte_array::<8>().to_vec(),
            perceptual_hash: Bit::from_bytes(&random_byte_array::<18>()),
            file_name: format!("test_image_{}.png", index),
            image_format: ImageFormat::PNG,
            res_width: 0,
            res_height: 0,
        }
    }

    #[test]
    #[ignore = "This test is slow"]
    fn create_test_images() {
        let db = test_db();
        let test_images = (0..10000).map(create_insert_row).collect::<Vec<_>>();
        let test_images2 = (10000..20000).map(create_insert_row).collect::<Vec<_>>();
        let test_images3 = (20000..30000).map(create_insert_row).collect::<Vec<_>>();
        let mut conn = db.get_connection().unwrap();
        insert_test_image(&mut conn, &test_images);
        insert_test_image(&mut conn, &test_images2);
        insert_test_image(&mut conn, &test_images3);
    }
}
