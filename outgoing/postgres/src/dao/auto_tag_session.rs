use crate::models::auto_tag_session::{AutoTagSessionInsertRow, AutoTagSessionRow};
use crate::models::auto_tag_session_image::{AutoTagSessionImageInsertRow, AutoTagSessionImageRow};
use crate::models::auto_tag_session_image_option::{
    AutoTagSessionImageOptionInsertRow, AutoTagSessionImageOptionRow,
};
use crate::schema::auto_tag_session::dsl::auto_tag_session;
use crate::schema::auto_tag_session_image::dsl as auto_tag_session_image_dsl;
use crate::schema::auto_tag_session_image::dsl::auto_tag_session_image;
use crate::schema::auto_tag_session_image_option::dsl::auto_tag_session_image_option;
use anyhow::{Context, Error};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

pub struct AutoTagDao<'c> {
    connection: &'c mut diesel::PgConnection,
}

impl<'c> AutoTagDao<'c> {
    pub fn new(connection: &'c mut diesel::PgConnection) -> Self {
        Self { connection }
    }

    pub fn get_all_auto_tag_sessions(
        &mut self,
    ) -> Result<Vec<AutoTagSessionRow>, Error> {
        auto_tag_session.load(self.connection)
            .context("Failed to load auto tag sessions from database")
    }

    pub fn insert_auto_tag_session(
        &mut self,
        insert_row: &AutoTagSessionInsertRow,
    ) -> Result<AutoTagSessionRow, Error> {
        diesel::insert_into(auto_tag_session)
            .values(insert_row)
            .returning(AutoTagSessionRow::as_returning())
            .get_result(self.connection)
            .context("Failed to insert auto tag session into database")
    }

    pub fn get_all_auto_tag_session_images(
        &mut self,
        session_id: i64,
    ) -> Result<Vec<AutoTagSessionImageRow>, Error> {
        auto_tag_session_image
            .select(AutoTagSessionImageRow::as_select())
            .filter(auto_tag_session_image_dsl::session_id.eq(session_id))
            .load(self.connection)
            .context("Failed to load auto tag session images from database")
    }

    pub fn insert_auto_tag_session_images(
        &mut self,
        insert_rows: &[AutoTagSessionImageInsertRow],
    ) -> Result<Vec<AutoTagSessionImageRow>, Error> {
        diesel::insert_into(auto_tag_session_image)
            .values(insert_rows)
            .returning(AutoTagSessionImageRow::as_returning())
            .load(self.connection)
            .context("Failed to insert auto tag session images into database")
    }

    pub fn get_all_auto_tag_session_image_options(
        &mut self,
        session_id: i64,
    ) -> Result<Vec<AutoTagSessionImageOptionRow>, Error> {
        auto_tag_session_image_option
            .select(AutoTagSessionImageOptionRow::as_select())
            .inner_join(auto_tag_session_image)
            .filter(auto_tag_session_image_dsl::session_id.eq(session_id))
            .load(self.connection)
            .context("Failed to load auto tag session image options from database")
    }

    pub fn insert_auto_tag_session_image_options(
        &mut self,
        insert_rows: &[AutoTagSessionImageOptionInsertRow],
    ) -> Result<Vec<AutoTagSessionImageOptionRow>, Error> {
        diesel::insert_into(auto_tag_session_image_option)
            .values(insert_rows)
            .returning(AutoTagSessionImageOptionRow::as_returning())
            .load(self.connection)
            .context("Failed to insert auto tag session image options into database")
    }
}

#[cfg(test)]
mod test {
    use crate::dao::test::{
        insert_test_auto_tag_session, insert_test_auto_tag_session_image,
        insert_test_auto_tag_session_image_option, insert_test_image, insert_test_user,
    };
    use crate::dao::Dao;
    use crate::models::auto_tag_session::AutoTagSessionInsertRow;
    use crate::models::auto_tag_session_image::AutoTagSessionImageInsertRow;
    use crate::models::auto_tag_session_image_option::AutoTagSessionImageOptionInsertRow;
    use crate::models::{AutoTagStatus, ReverseLookupSite, SourceSiteName};
    use crate::test::test_db;
    use diesel::Connection;

    #[test]
    #[serial_test::serial]
    fn test_insert_auto_tag_session() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            c.auto_tag_dao()
                .insert_auto_tag_session(&AutoTagSessionInsertRow {
                    user_id: user.id,
                    lookup_site: ReverseLookupSite::IQDB,
                    closed_at: None,
                })
        });
        println!("session: {:?}", result);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_all_auto_tag_sessions() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let _session = insert_test_auto_tag_session(c, user.id)?;
            c.auto_tag_dao().get_all_auto_tag_sessions()
        });
        for result in results {
            println!("session: {:?}", result);
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_insert_auto_tag_session_images() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let session = insert_test_auto_tag_session(c, user.id)?;
            let image = insert_test_image(c)?;
            c.auto_tag_dao()
                .insert_auto_tag_session_images(&[AutoTagSessionImageInsertRow {
                    session_id: session.id,
                    image_id: image.id,
                    status: AutoTagStatus::PENDING,
                }])
        });
        for result in results {
            println!("session image: {:?}", result);
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_get_all_auto_tag_session_images() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let session = insert_test_auto_tag_session(c, user.id)?;
            c.auto_tag_dao().get_all_auto_tag_session_images(session.id)
        });
        for result in results {
            println!("session image: {:?}", result);
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_get_all_auto_tag_session_image_options() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let image = insert_test_image(c)?;
            let session = insert_test_auto_tag_session(c, user.id)?;
            let session_image = insert_test_auto_tag_session_image(c, session.id, image.id)?;
            let _options = insert_test_auto_tag_session_image_option(c, session_image.id)?;
            c.auto_tag_dao()
                .get_all_auto_tag_session_image_options(session.id)
        });
        for result in results {
            println!("session: {:?}", result);
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_insert_auto_tag_session_image_options() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let user = insert_test_user(c)?;
            let session = insert_test_auto_tag_session(c, user.id)?;
            let image = insert_test_image(c)?;
            let session_image = insert_test_auto_tag_session_image(c, session.id, image.id)?;
            c.auto_tag_dao().insert_auto_tag_session_image_options(&[
                AutoTagSessionImageOptionInsertRow {
                    session_image_id: session_image.id,
                    source_site: SourceSiteName::GELBOORU,
                    source_url: "example.com".to_string(),
                    certainty: 0.9,
                },
            ])
        });
        for result in results {
            println!("session uncertainty: {:?}", result);
        }
    }
}
