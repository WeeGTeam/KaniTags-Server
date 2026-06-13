use crate::dao::Dao;
use crate::models::image::ImageRow;
use crate::models::import_session::ImportSessionInsertRow;
use crate::models::import_session_image::ImportSessionImageInsertRow;
use crate::models::user_image::UserImageInsertRow;
use crate::Postgres;
use diesel::Connection;
use kani_domain_api_model::image::{CreatePantsuImage, PantsuImage};
use kani_domain_api_model::image_hash::hash_to_hex;
use kani_domain_api_model::image_id::{ImageId, ImageIdHash};
use kani_domain_api_model::image_search::ImageSearchFilter;
use kani_domain_api_model::import::ImportSessionId;
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::ImageDatabase;
use tracing::debug;

impl ImageDatabase for Postgres {
    fn get_image_by_image_id(&self, image_id: ImageId) -> Result<Option<PantsuImage>, anyhow::Error> {
        debug!("Getting image by image id: {:?}", image_id);
        let mut connection = self.get_connection()?;
        let image_row =
            connection.transaction(|conn| conn.image_dao().get_image_by_id(*image_id))?;
        debug!("Got image by image id: {:?}: {}", image_id, image_row.is_some());
        Ok(image_row.map(TryInto::try_into).transpose()?)
    }

    fn get_image_by_image_id_hash(&self, image_id_hash: &ImageIdHash) -> Result<Option<PantsuImage>, anyhow::Error> {
        debug!("Getting image by image id hash: {}", image_id_hash);
        let mut connection = self.get_connection()?;
        let image_row =
            connection.transaction(|conn| conn.image_dao().get_image_by_id_hash(&image_id_hash.0))?;
        debug!("Got image by image id hash: {}: {}", image_id_hash, image_row.is_some());
        Ok(image_row.map(TryInto::try_into).transpose()?)
    }

    fn get_images_by_image_ids(
        &self,
        user: &User,
        image_ids: &[ImageId]
    ) -> Result<Vec<PantsuImage>, anyhow::Error> {
        debug!("Getting {} images by image ids", image_ids.len());
        let mut connection = self.get_connection()?;
        let image_rows = connection.transaction(|conn|
            conn.image_dao().get_images_by_user_and_ids(user.id, &image_ids.iter().map(|id| id.0).collect::<Vec<_>>())
        )?;
        debug!("Got {} images by image ids", image_rows.len());
        Ok(image_rows.into_iter().map(TryInto::try_into).collect::<Result<Vec<PantsuImage>, anyhow::Error>>()?)
    }


    fn store_image(
        &self,
        user: &User,
        import_session_id: ImportSessionId,
        image: &CreatePantsuImage,
    ) -> Result<PantsuImage, anyhow::Error> {
        debug!("Storing image: {}", hash_to_hex(&image.id_hash));
        let mut connection = self.get_connection()?;
        let image_row = connection.transaction(|conn| {
            let image = conn.image_dao().insert_image(&image.into())?;
            let _session_images = conn.import_session_dao().insert_import_session_images(&[
                ImportSessionImageInsertRow {
                    import_id: *import_session_id,
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

    fn start_import_session(&self, user: &User) -> Result<ImportSessionId, anyhow::Error> {
        debug!("Starting import session for user: {}", user.user_name);
        let mut connection = self.get_connection()?;
        let row = connection.transaction(|conn| {
            conn.import_session_dao()
                .insert_import_session(&ImportSessionInsertRow { user_id: user.id })
        })?;
        debug!("Started import session with id: {}", row.id);
        Ok(row.into())
    }

    fn close_import_session(&self, import_session_id: ImportSessionId) -> Result<(), anyhow::Error> {
        debug!("Closing import session with id: {}", *import_session_id);
        let mut connection = self.get_connection()?;
        connection.transaction(|conn| {
            conn.import_session_dao()
                .close_import_session(*import_session_id)
        })?;
        debug!("Closed import session with id: {}", *import_session_id);
        Ok(())
    }

    fn search_images(&self, user: &User, filter: &ImageSearchFilter) -> Result<Vec<ImageIdHash>, anyhow::Error> {
        debug!("Starting image search for user '{}' and filter '{:?}'", user.user_name, filter);
        let mut connection = self.get_connection()?;
        let rows = connection.transaction(|conn| {{
            conn.image_dao().search_images(user.id, filter)
        }})?;
        debug!("Finished image search with {} results", rows.len());
        rows.into_iter().map(TryInto::try_into).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dao::test::{insert_test_image, insert_test_import_session, insert_test_user};
    use crate::test::test_db;
    use assertables::{assert_none, assert_ok, assert_some};
    use kani_domain_api_model::image_format::ImageFormat;
    use kani_domain_api_model::image_hash::IdHash;

    #[test]
    #[serial_test::serial]
    fn test_get_image_by_image_id() {
        let db = test_db();
        let mut connection = db.get_connection().unwrap();
        let db_image = insert_test_image(&mut connection).unwrap();
        assert_some!(assert_ok!(db.get_image_by_image_id(ImageId(db_image.id))));
    }

    #[test]
    #[serial_test::serial]
    fn test_get_image_by_image_id_hash() {
        let db = test_db();
        let mut connection = db.get_connection().unwrap();
        let db_image = insert_test_image(&mut connection).unwrap();
        assert_ok!(db.get_image_by_image_id_hash(&ImageIdHash(IdHash::try_from(db_image.id_hash).unwrap())));
    }

    #[test]
    #[serial_test::serial]
    fn test_get_error() {
        let db = test_db();
        assert_none!(assert_ok!(
            db.get_image_by_image_id_hash(&ImageIdHash([1, 2, 3, 4, 5, 6, 7, 8]))
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

        assert_ok!(db.store_image(&user, ImportSessionId(session.id), &create_image));
    }
}
