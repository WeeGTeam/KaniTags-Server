use crate::dao::Dao;
use crate::models::image::ImageRow;
use crate::models::import_session::ImportSessionInsertRow;
use crate::models::import_session_image::ImportSessionImageInsertRow;
use crate::models::user_image::UserImageInsertRow;
use crate::Postgres;
use diesel::Connection;
use kani_domain_api_model::image::{CreatePantsuImage, PantsuImage};
use kani_domain_api_model::image_hash::hash_to_hex;
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::import::ImportSession;
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::ImageDatabase;
use tracing::debug;

mod converter;

impl ImageDatabase for Postgres {
    fn get_image_by_image_id(&self, image_id: &ImageId) -> Result<Option<PantsuImage>, anyhow::Error> {
        debug!("Getting image by image id: {}", image_id);
        let mut connection = self.get_connection()?;
        let image_row =
            connection.transaction(|conn| conn.image_dao().get_image_by_id_hash(&image_id.0))?;
        debug!("Got image by image id: {}: {}", image_id, image_row.is_some());
        Ok(image_row.map(TryInto::try_into).transpose()?)
    }

    fn store_image(
        &self,
        user: &User,
        import_session_id: i64,
        image: &CreatePantsuImage,
    ) -> Result<PantsuImage, anyhow::Error> {
        debug!("Storing image: {}", hash_to_hex(&image.id_hash));
        let mut connection = self.get_connection()?;
        let image_row = connection.transaction(|conn| {
            let session = conn
                .import_session_dao()
                .get_import_session_by_id_and_user(import_session_id, user.id)?
                .ok_or_else(|| anyhow::anyhow!("Import session not found"))?;
            let image = conn.image_dao().insert_image(&image.into())?;
            let _session_images = conn.import_session_dao().insert_import_session_images(&[
                ImportSessionImageInsertRow {
                    import_id: session.id,
                    image_id: image.id,
                },
            ])?;
            let _user_image = conn.image_dao().insert_user_image(&UserImageInsertRow {
                user_id: user.id,
                image_id: image.id,
            })?;
            Ok::<ImageRow, anyhow::Error>(image)
        })?;
        debug!("Stored image: {}", hash_to_hex(&image.id_hash));
        image_row.try_into()
    }

    fn start_import_session(&self, user: &User) -> Result<ImportSession, anyhow::Error> {
        debug!("Starting import session for user: {}", user.id);
        let mut connection = self.get_connection()?;
        let row = connection.transaction(|conn| {
            conn.import_session_dao()
                .insert_import_session(&ImportSessionInsertRow { user_id: user.id })
        })?;
        debug!("Started import session with id: {}", row.id);
        Ok(row.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dao::test::{insert_test_image, insert_test_import_session, insert_test_user};
    use crate::test::test_db;
    use assertables::{assert_none, assert_ok};
    use kani_domain_api_model::image_format::ImageFormat;
    use kani_domain_api_model::image_hash::IdHash;

    #[test]
    #[serial_test::serial]
    fn test_get_ok() {
        let db = test_db();
        let mut connection = db.get_connection().unwrap();
        let db_image = insert_test_image(&mut connection).unwrap();
        assert_ok!(db.get_image_by_image_id(&ImageId(IdHash::try_from(db_image.id_hash).unwrap())));
    }

    #[test]
    #[serial_test::serial]
    fn test_get_error() {
        let db = test_db();
        assert_none!(assert_ok!(
            db.get_image_by_image_id(&ImageId([1, 2, 3, 4, 5, 6, 7, 8]))
        ))
    }

    #[test]
    #[serial_test::serial]
    fn test_store_ok() {
        let db = test_db();
        let mut connection = db.get_connection().unwrap();
        let user_row = insert_test_user(&mut connection).unwrap();
        let session = insert_test_import_session(&mut connection, user_row.id).unwrap();
        let user = User {
            id: user_row.id,
            user_name: user_row.user_name,
            display_name: user_row.display_name,
        };
        let create_image = CreatePantsuImage {
            id_hash: [1, 2, 3, 4, 5, 6, 7, 8],
            perceptual_hash: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, ],
            upload_filename: "test_file_name".to_string(),
            format: ImageFormat::PNG,
            dimensions: (0, 0),
        };

        assert_ok!(db.store_image(&user, session.id, &create_image));
    }
}
