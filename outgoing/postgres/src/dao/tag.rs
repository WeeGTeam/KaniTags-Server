use crate::models::image_tag::{ImageTagInsertRow, ImageTagRow};
use crate::models::tag::{TagInsertRow, TagRow};
use crate::schema::image::dsl as image_dsl;
use crate::schema::image::dsl::image;
use crate::schema::image_tag::dsl::image_tag;
use crate::schema::tag::dsl as tag_dsl;
use crate::schema::tag::dsl::tag;
use anyhow::{Context, Error};
use diesel::{BoolExpressionMethods, ExpressionMethods};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

pub struct TagDao<'c> {
    connection: &'c mut diesel::PgConnection,
}

impl<'c> TagDao<'c> {
    pub fn new(connection: &'c mut diesel::PgConnection) -> Self {
        TagDao { connection }
    }

    pub fn insert_tags(&mut self, insert_rows: &[TagInsertRow]) -> Result<Vec<TagRow>, Error> {
        diesel::insert_into(tag)
            .values(insert_rows)
            .returning(TagRow::as_returning())
            .get_results(self.connection)
            .context("Failed to insert tag into database")
    }

    pub fn get_all_tags(&mut self) -> Result<Vec<TagRow>, Error> {
        tag.load(self.connection)
            .context("Failed to load tags from database")
    }

    pub fn get_tags_by_name_and_type(&mut self, tags: &[TagInsertRow]) -> Result<Vec<TagRow>, Error> {
        let mut query = tag
            .select(TagRow::as_select())
            .into_boxed();
        query = tags.iter().fold(query, |query, current_tag| {
            query.filter(tag_dsl::tag_type.eq(&current_tag.tag_type).and(tag_dsl::tag_name.eq(&current_tag.tag_name)))
        });
        query.load(self.connection)
            .context("Failed to load tags by their name and type from database")
    }

    pub fn insert_image_tags(
        &mut self,
        insert_rows: &[ImageTagInsertRow],
    ) -> Result<Vec<ImageTagRow>, Error> {
        diesel::insert_into(image_tag)
            .values(insert_rows)
            .returning(ImageTagRow::as_returning())
            .get_results(self.connection)
            .context("Failed to insert image tag into database")
    }

    pub fn get_all_image_tags_by_image(
        &mut self,
        image_id_hash: &[u8],
    ) -> Result<Vec<(ImageTagRow, TagRow)>, Error> {
        image_tag
            .inner_join(image)
            .filter(image_dsl::id_hash.eq(image_id_hash))
            .inner_join(tag)
            .select((ImageTagRow::as_select(), TagRow::as_select()))
            .load(self.connection)
            .context("Failed to load image tags from database")
    }
}

#[cfg(test)]
mod test {
    use crate::dao::test::{insert_test_image, insert_test_image_tag, insert_test_tag, insert_test_tag_with, insert_test_user};
    use crate::dao::Dao;
    use crate::models::tag::TagInsertRow;
    use crate::models::TagType;
    use crate::test::test_db;
    use assertables::assert_len_eq_x;
    use diesel::Connection;

    #[test]
    #[serial_test::serial]
    fn test_insert_tags() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let results = conn.test_transaction(|c| {
            c.tag_dao().insert_tags(&[TagInsertRow {
                tag_type: TagType::CHARACTER,
                tag_name: "Megumin".to_string(),
            }])
        });
        assert_len_eq_x!(results, 1);
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
    fn test_get_tag_by_name_and_type() {
        let postgres = test_db();
        let mut conn = postgres.get_connection().unwrap();
        let result = conn.test_transaction(|c| {
            let test_tag_type = TagType::CHARACTER;
            let test_tag_name = "Megumin";
            let _tag = insert_test_tag_with(c, test_tag_type.clone(), test_tag_name.to_owned())?;

            c.tag_dao().get_tags_by_name_and_type(&[TagInsertRow{ tag_type: test_tag_type, tag_name: test_tag_name.to_owned() }])
        });
        assert_len_eq_x!(result, 1);
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
            c.tag_dao().get_all_image_tags_by_image(image.id_hash.as_ref())
        });
        assert_len_eq_x!(result, 1);
    }
}
