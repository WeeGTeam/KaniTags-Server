use crate::models::collection::{CollectionInsertRow, CollectionRow};
use crate::models::collection_image::{CollectionImageInsertRow, CollectionImageRow};
use crate::schema::collection::dsl as collection_dsl;
use crate::schema::collection::dsl::collection;
use crate::schema::collection_image::dsl as collection_image_dsl;
use crate::schema::collection_image::dsl::collection_image;
use anyhow::{Context, Error};
use diesel::{BoolExpressionMethods, ExpressionMethods};
use diesel::{OptionalExtension, QueryDsl};
use diesel::{RunQueryDsl, SelectableHelper};

pub struct CollectionDao<'c> {
    connection: &'c mut diesel::PgConnection,
}

impl<'c> CollectionDao<'c> {
    pub fn new(connection: &'c mut diesel::PgConnection) -> Self {
        Self { connection }
    }

    pub fn insert_collection(
        &mut self,
        insert_row: &CollectionInsertRow,
    ) -> Result<CollectionRow, Error> {
        diesel::insert_into(collection)
            .values(insert_row)
            .returning(CollectionRow::as_returning())
            .get_result(self.connection)
            .context("Failed to insert collection into database")
    }

    pub fn delete_collection(&mut self, user_id: i64, id: i64) -> Result<CollectionRow, Error> {
        diesel::delete(collection.filter(collection_dsl::id.eq(id).and(collection_dsl::user_id.eq(user_id))))
            .returning(CollectionRow::as_returning())
            .get_result(self.connection)
            .context("Failed to delete collection from database")
    }

    pub fn get_collection_by_user_and_id(
        &mut self,
        user_id: i64,
        id: i64
    ) -> Result<Option<CollectionRow>, Error> {
        collection
            .select(CollectionRow::as_select())
            .filter(collection_dsl::user_id.eq(user_id).and(collection_dsl::id.eq(id)))
            .first(self.connection)
            .optional()
            .context("Failed to get collection by user and id")
    }

    pub fn get_collection_by_user_and_name(
        &mut self,
        user_id: i64,
        name: &str,
    ) -> Result<Option<CollectionRow>, Error> {
        collection
            .select(CollectionRow::as_select())
            .filter(collection_dsl::user_id.eq(user_id).and(collection_dsl::name.eq(name)))
            .first(self.connection)
            .optional()
            .context("Failed to get collection by user and name")
    }

    pub fn get_collections_by_user(&mut self, user_id: i64) -> Result<Vec<CollectionRow>, Error> {
        collection
            .select(CollectionRow::as_select())
            .filter(collection_dsl::user_id.eq(user_id))
            .load(self.connection)
            .context("Failed to load collections from database")
    }

    pub fn insert_collection_images(
        &mut self,
        insert_rows: &[CollectionImageInsertRow],
    ) -> Result<Vec<CollectionImageRow>, Error> {
        diesel::insert_into(collection_image)
            .values(insert_rows)
            .returning(CollectionImageRow::as_returning())
            .on_conflict_do_nothing()
            .get_results(self.connection)
            .context("Failed to insert collection images into database")
    }

    pub fn delete_collection_images(
        &mut self,
        collection_id: i64,
        image_ids: &[i64],
    ) -> Result<Vec<CollectionImageRow>, Error> {
        diesel::delete(collection_image.filter(collection_image_dsl::image_id.eq_any(image_ids).and(collection_image_dsl::collection_id.eq(collection_id))))
            .returning(CollectionImageRow::as_returning())
            .get_results(self.connection)
            .context("Failed to delete collection images from the database")
    }

    pub fn get_all_collection_images(
        &mut self,
        collection_id: i64,
    ) -> Result<Vec<CollectionImageRow>, Error> {
        collection_image
            .filter(collection_image_dsl::collection_id.eq(collection_id))
            .load(self.connection)
            .context("Failed to load collection images from database")
    }
}

#[cfg(test)]
mod test {
    use crate::dao::test::{
        insert_test_collection, insert_test_collection_image, insert_test_image, insert_test_user,
    };
    use crate::dao::Dao;
    use crate::models::collection::CollectionInsertRow;
    use crate::models::collection_image::CollectionImageInsertRow;
    use crate::test::test_db;
    use assertables::{assert_len_eq_x, assert_some};
    use diesel::Connection;

    #[test]
    #[serial_test::serial]
    fn test_insert_collection() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let _result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            c.collection_dao().insert_collection(&CollectionInsertRow {
                user_id: user.id,
                name: "test_collection".to_string(),
            })
        });
    }

    #[test]
    #[serial_test::serial]
    fn test_delete_collection() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let _result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let collection_row = insert_test_collection(c, user.id)?;
            c.collection_dao().delete_collection(user.id, collection_row.id)
        });
    }

    #[test]
    #[serial_test::serial]
    fn test_get_collection_by_user_and_id() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let collection_row = insert_test_collection(c, user.id)?;
            c.collection_dao().get_collection_by_user_and_id(user.id, collection_row.id)
        });
        assert_some!(result);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_collection_by_user_and_name() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let collection_row = insert_test_collection(c, user.id)?;
            c.collection_dao().get_collection_by_user_and_name(user.id, &collection_row.name)
        });
        assert_some!(result);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_collections_by_user() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let _collection = insert_test_collection(c, user.id)?;
            c.collection_dao().get_collections_by_user(user.id)
        });
        assert_len_eq_x!(results, 1);
    }

    #[test]
    #[serial_test::serial]
    fn test_insert_collection_image() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let image = insert_test_image(c)?;
            let user = insert_test_user(c)?;
            let collection_row = insert_test_collection(c, user.id)?;
            c.collection_dao()
                .insert_collection_images(&[CollectionImageInsertRow {
                    image_id: image.id,
                    collection_id: collection_row.id,
                }])
        });
        assert_len_eq_x!(results, 1);
    }

    #[test]
    #[serial_test::serial]
    fn test_insert_duplicate_collection_image_without_conflict() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let image = insert_test_image(c)?;
            let user = insert_test_user(c)?;
            let collection_row = insert_test_collection(c, user.id)?;
            let _images = c.collection_dao()
                .insert_collection_images(&[CollectionImageInsertRow {
                    image_id: image.id,
                    collection_id: collection_row.id,
                }])?;
            c.collection_dao()
                .insert_collection_images(&[CollectionImageInsertRow {
                    image_id: image.id,
                    collection_id: collection_row.id,
                }])
        });
        assert_len_eq_x!(results, 0);
    }

    #[test]
    #[serial_test::serial]
    #[should_panic]
    fn test_insert_collection_image_with_inexistent_image() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let _results = conn.test_transaction(|c| {
            let image = insert_test_image(c)?;
            let user = insert_test_user(c)?;
            let collection_row = insert_test_collection(c, user.id)?;
            let _images = c.collection_dao()
                .insert_collection_images(&[CollectionImageInsertRow {
                    image_id: image.id,
                    collection_id: collection_row.id,
                }])?;
            c.collection_dao()
                .insert_collection_images(&[CollectionImageInsertRow {
                    image_id: -1,
                    collection_id: collection_row.id,
                }])
        });
    }

    #[test]
    #[serial_test::serial]
    fn test_delete_collection_images() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let image = insert_test_image(c)?;
            let image2 = insert_test_image(c)?;
            let image3 = insert_test_image(c)?;
            let image_not_in_collection = insert_test_image(c)?;
            let user = insert_test_user(c)?;
            let collection_row = insert_test_collection(c, user.id)?;
            c.collection_dao()
                .insert_collection_images(&[
                    CollectionImageInsertRow {
                        image_id: image.id,
                        collection_id: collection_row.id,
                    },
                    CollectionImageInsertRow {
                        image_id: image2.id,
                        collection_id: collection_row.id,
                    },
                    CollectionImageInsertRow {
                        image_id: image3.id,
                        collection_id: collection_row.id,
                    }
                ])?;
            c.collection_dao()
                .delete_collection_images(collection_row.id, &[image.id, image_not_in_collection.id])
        });
        assert_len_eq_x!(results, 1);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_all_collection_images() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let collection_row = insert_test_collection(c, user.id)?;
            let image = insert_test_image(c)?;
            let _collection_image = insert_test_collection_image(c, collection_row.id, image.id)?;
            c.collection_dao().get_all_collection_images(collection_row.id)
        });
        assert_len_eq_x!(results, 1);
    }
}
