use crate::dao::Dao;
use crate::Postgres;
use diesel::Connection;
use kani_domain_api_model::image::{CreatePantsuImage, PantsuImage};
use kani_domain_api_model::image_hash::hash_to_hex;
use kani_domain_api_model::image_hash::IdHash;
use kani_domain_api_outgoing::database::{Database, ImageDatabase};
use tracing::debug;

pub mod converter;

impl Database for Postgres {}

impl ImageDatabase for Postgres {
    fn get_image_by_id_hash(&self, id_hash: &IdHash) -> Result<Option<PantsuImage>, anyhow::Error> {
        debug!("Getting image by id hash: {}", hash_to_hex(id_hash));
        let mut connection = self.get_connection()?;
        let image_row =
            connection.transaction(|conn| conn.image_dao().get_image_by_id_hash(id_hash))?;
        debug!(
            "Got image by id hash: {}: {}",
            hash_to_hex(id_hash),
            image_row.is_some()
        );
        Ok(image_row.map(TryInto::try_into).transpose()?)
    }

    fn store_image(&self, image: &CreatePantsuImage) -> Result<PantsuImage, anyhow::Error> {
        debug!("Storing image: {}", image.id);
        let mut connection = self.get_connection()?;
        let image_row =
            connection.transaction(|conn| conn.image_dao().insert_image(&image.into()))?;
        debug!("Stored image: {}", image.id);
        image_row.try_into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dao::test::insert_test_image;
    use crate::test::test_db;
    use assertables::{assert_none, assert_ok};
    use kani_domain_api_model::image_format::ImageFormat;
    use kani_domain_api_model::image_id::ImageId;

    #[test]
    #[serial_test::serial]
    fn test_get_ok() {
        let db = test_db();
        let mut connection = db.get_connection().unwrap();
        let db_image = insert_test_image(&mut connection).unwrap();
        assert_ok!(db.get_image_by_id_hash(&IdHash::try_from(db_image.id_hash).unwrap()));
    }

    #[test]
    #[serial_test::serial]
    fn test_get_error() {
        let db = test_db();
        assert_none!(assert_ok!(
            db.get_image_by_id_hash(&[1, 2, 3, 4, 5, 6, 7, 8])
        ))
    }

    #[test]
    #[serial_test::serial]
    fn test_store_ok() {
        let db = test_db();
        let create_image = CreatePantsuImage {
            id: ImageId::new(
                [1, 2, 3, 4, 5, 6, 7, 8],
                [
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                ],
            ),
            upload_filename: "test_file_name".to_string(),
            format: ImageFormat::PNG,
            dimensions: (0, 0),
        };
        assert_ok!(db.store_image(&create_image));
    }
}
