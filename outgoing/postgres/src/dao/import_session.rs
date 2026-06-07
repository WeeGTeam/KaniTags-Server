use crate::models::import_session::{ImportSessionInsertRow, ImportSessionRow};
use crate::models::import_session_image::{ImportSessionImageInsertRow, ImportSessionImageRow};
use crate::schema::import_session::dsl as import_session_dsl;
use crate::schema::import_session::dsl::import_session;
use crate::schema::import_session_image::dsl as import_session_image_dsl;
use crate::schema::import_session_image::dsl::import_session_image;
use anyhow::Context;
use anyhow::Error;
use diesel::{BoolExpressionMethods, ExpressionMethods, OptionalExtension};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

pub struct ImportSessionDao<'c> {
    connection: &'c mut diesel::PgConnection,
}

impl<'c> ImportSessionDao<'c> {
    pub fn new(connection: &'c mut diesel::PgConnection) -> Self {
        Self { connection }
    }

    pub fn insert_import_session(
        &mut self,
        insert_row: &ImportSessionInsertRow,
    ) -> Result<ImportSessionRow, Error> {
        diesel::insert_into(import_session)
            .values(insert_row)
            .returning(ImportSessionRow::as_returning())
            .get_result(self.connection)
            .context("Failed to insert import session into database")
    }

    pub fn close_import_session(&mut self, import_id: i64) -> Result<ImportSessionRow, Error> {
        diesel::update(import_session.filter(import_session_dsl::id.eq(import_id)))
            .set(import_session_dsl::closed_at.eq(diesel::dsl::now))
            .returning(ImportSessionRow::as_returning())
            .get_result(self.connection)
            .context("Failed to close import session")
    }

    pub fn get_import_session_by_id_and_user(&mut self, import_id: i64, user_id: i64) -> Result<Option<ImportSessionRow>, Error> {
        import_session.select(ImportSessionRow::as_select())
            .filter(import_session_dsl::id.eq(import_id).and(import_session_dsl::user_id.eq(user_id)))
            .get_result(self.connection)
            .optional()
            .context("Failed to get import session from database")
    }

    pub fn get_all_import_sessions_of_user(&mut self, user_id: i64) -> Result<Vec<ImportSessionRow>, Error> {
        import_session.select(ImportSessionRow::as_select())
            .filter(import_session_dsl::user_id.eq(user_id))
            .get_results(self.connection)
            .context("Failed to get all import sessions from database")
    }

    pub fn insert_import_session_images(
        &mut self,
        insert_rows: &[ImportSessionImageInsertRow],
    ) -> Result<Vec<ImportSessionImageRow>, Error> {
        diesel::insert_into(import_session_image)
            .values(insert_rows)
            .returning(ImportSessionImageRow::as_returning())
            .get_results(self.connection)
            .context("Failed to insert import session images into database")
    }

    pub fn get_all_import_session_images(
        &mut self,
        import_session_id: i64,
    ) -> Result<Vec<ImportSessionImageRow>, Error> {
        import_session_image
            .select(ImportSessionImageRow::as_select())
            .filter(import_session_image_dsl::import_id.eq(import_session_id))
            .load(self.connection)
            .context("Failed to get images by import session from database")
    }
}

#[cfg(test)]
mod test {
    use crate::dao::test::{insert_test_image, insert_test_import_session, insert_test_import_session_image, insert_test_user};
    use crate::dao::Dao;
    use crate::models::import_session::ImportSessionInsertRow;
    use crate::models::import_session_image::ImportSessionImageInsertRow;
    use crate::test::test_db;
    use assertables::{assert_len_eq_x, assert_some};
    use diesel::Connection;

    #[test]
    #[serial_test::serial]
    fn test_insert_import_session() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let _result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            c.import_session_dao().insert_import_session(&ImportSessionInsertRow {
                user_id: user.id,
            })
        });
    }

    #[test]
    #[serial_test::serial]
    fn test_close_import_session() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let session = c.import_session_dao().insert_import_session(&ImportSessionInsertRow {
                user_id: user.id,
            })?;
            c.import_session_dao().close_import_session(session.id)
        });
        assert_some!(result.closed_at);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_import_session_by_id() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let import_session = insert_test_import_session(c, user.id)?;
            c.import_session_dao().get_import_session_by_id_and_user(import_session.id, user.id)
        });
        assert_some!(result);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_all_import_sessions_by_user() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let user2 = insert_test_user(c)?;
            let _import_session = insert_test_import_session(c, user.id)?;
            let _import_session2 = insert_test_import_session(c, user2.id)?;
            c.import_session_dao().get_all_import_sessions_of_user(user.id)
        });
        assert_len_eq_x!(result, 1);
    }

    #[test]
    #[serial_test::serial]
    fn test_insert_import_session_images() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let image = insert_test_image(c)?;
            let import_session = insert_test_import_session(c, user.id)?;
            c.import_session_dao().insert_import_session_images(&[ImportSessionImageInsertRow {
                import_id: import_session.id,
                image_id: image.id,
            }])
        });
        assert_len_eq_x!(result, 1);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_all_import_session_images() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let import_session = insert_test_import_session(c, user.id)?;
            let image = insert_test_image(c)?;
            let _import_session_image = insert_test_import_session_image(c, import_session.id, image.id)?;
            c.import_session_dao().get_all_import_session_images(import_session.id)
        });
        assert_len_eq_x!(results, 1);
    }
}
