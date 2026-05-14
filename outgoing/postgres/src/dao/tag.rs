use crate::models::image_tag::{ImageTagInsertRow, ImageTagRow};
use crate::models::tag::{TagInsertRow, TagRow};
use crate::schema::image_tag::dsl as image_tag_dsl;
use crate::schema::image_tag::dsl::image_tag;
use crate::schema::tag::dsl::tag;
use anyhow::{Context, Error};
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

pub struct TagDao<'c> {
    connection: &'c mut diesel::PgConnection,
}

impl<'c> TagDao<'c> {
    pub fn new(connection: &'c mut diesel::PgConnection) -> Self {
        TagDao { connection }
    }

    pub fn insert_tag(&mut self, insert_row: &TagInsertRow) -> Result<TagRow, Error> {
        diesel::insert_into(tag)
            .values(insert_row)
            .returning(TagRow::as_returning())
            .get_result(self.connection)
            .context("Failed to insert tag into database")
    }

    pub fn get_all_tags(&mut self) -> Result<Vec<TagRow>, Error> {
        tag.load(self.connection)
            .context("Failed to load tags from database")
    }

    pub fn insert_image_tag(
        &mut self,
        insert_row: &ImageTagInsertRow,
    ) -> Result<ImageTagRow, Error> {
        diesel::insert_into(image_tag)
            .values(insert_row)
            .returning(ImageTagRow::as_returning())
            .get_result(self.connection)
            .context("Failed to insert image tag into database")
    }

    pub fn get_all_image_tags_by_image(
        &mut self,
        image_id: i64,
    ) -> Result<Vec<ImageTagRow>, Error> {
        image_tag
            .filter(image_tag_dsl::image_id.eq(image_id))
            .select(ImageTagRow::as_select())
            .load(self.connection)
            .context("Failed to load image tags from database")
    }
}

#[cfg(test)]
mod test {
    use crate::dao::test::{
        insert_test_image, insert_test_image_tag, insert_test_tag, insert_test_user,
    };
    use crate::dao::Dao;
    use crate::models::tag::TagInsertRow;
    use crate::models::TagType;
    use crate::test::test_db;
    use assertables::assert_len_eq_x;
    use diesel::Connection;

    #[test]
    #[serial_test::serial]
    fn test_insert_tag() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let _result = conn.test_transaction(|c| {
            c.tag_dao().insert_tag(&TagInsertRow {
                tag_type: TagType::CHARACTER,
                tag_name: "Megumin".to_string(),
            })
        });
    }

    #[test]
    #[serial_test::serial]
    fn test_get_all_tags() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            let _tag = insert_test_tag(c)?;
            c.tag_dao().get_all_tags()
        });
        assert_len_eq_x!(results, 1);
    }

    #[test]
    #[serial_test::serial]
    fn test_get_user_by_user_name() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let image = insert_test_image(c)?;
            let tag = insert_test_tag(c)?;
            let user = insert_test_user(c)?;
            let _image_tag = insert_test_image_tag(c, image.id, tag.id, Some(user.id))?;
            c.tag_dao().get_all_image_tags_by_image(image.id)
        });
        assert_len_eq_x!(result, 1);
    }
}
