use crate::models::collection::{CollectionInsertRow, CollectionRow};
use crate::models::collection_image::{CollectionImageInsertRow, CollectionImageRow};
use crate::schema::collection::dsl::collection;
use crate::schema::collection_image::dsl as collection_image_dsl;
use crate::schema::collection_image::dsl::collection_image;
use anyhow::{Context, Error};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
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

    pub fn get_all_collections(&mut self) -> Result<Vec<CollectionRow>, Error> {
        collection
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
            .get_results(self.connection)
            .context("Failed to insert collection images into database")
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
    use crate::dao::Dao;
    use crate::dao::test::{
        insert_test_collection, insert_test_collection_image, insert_test_image, insert_test_user,
    };
    use crate::models::collection::CollectionInsertRow;
    use crate::models::collection_image::CollectionImageInsertRow;
    use crate::test::test_db;
    use diesel::Connection;

    #[test]
    #[serial_test::serial]
    fn test_insert_collection() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            c.collection_dao().insert_collection(&CollectionInsertRow {
                user_id: user.id,
                name: "test_collection".to_string(),
            })
        });
        println!("collection: {:?}", result);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_all_collections() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let _collection = insert_test_collection(c, user.id)?;
            c.collection_dao().get_all_collections()
        });
        for result in results {
            println!("collection: {:?}", result);
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_insert_collection_image() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let image = insert_test_image(c)?;
            let user = insert_test_user(c)?;
            let collection = insert_test_collection(c, user.id)?;
            c.collection_dao()
                .insert_collection_images(&[CollectionImageInsertRow {
                    image_id: image.id,
                    collection_id: collection.id,
                }])
        });
        for result in results {
            println!("collection image: {:?}", result);
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_get_all_collection_images() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let collection = insert_test_collection(c, user.id)?;
            let image = insert_test_image(c)?;
            let _collection_image = insert_test_collection_image(c, collection.id, image.id)?;
            c.collection_dao().get_all_collection_images(collection.id)
        });
        for result in results {
            println!("collection image: {:?}", result);
        }
    }
}
