mod converter;

use crate::dao::Dao;
use crate::database::converter::{FromDomain, TryToDomain};
use crate::models::image_tag::ImageTagInsertRow;
use crate::models::tag::TagInsertRow;
use crate::Postgres;
use diesel::Connection;
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::tag::image_tag::ImageTag;
use kani_domain_api_model::tag::{NewTag, Tag};
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::tag_database::TagDatabase;
use tracing::debug;

impl TagDatabase for Postgres {
    fn get_all_tags(&self) -> Result<Vec<Tag>, anyhow::Error> {
        debug!("Getting all tags");
        self.get_connection()?
            .transaction(|conn| conn.tag_dao().get_all_tags())?
            .try_to_domain()
    }

    fn get_tags_create_if_missing(&self, new_tags: Vec<NewTag>) -> Result<Vec<Tag>, anyhow::Error> {
        debug!("Getting requested tags, creating those that do not yet exist");
        let tags: Vec<TagInsertRow> = FromDomain::from_domain(new_tags);
        self.get_connection()?
            .transaction(|conn| {
                let created_tags = conn.tag_dao().insert_tags_if_missing(&tags)?;
                debug!("Created missing tags: {:?}", created_tags);

                conn.tag_dao()
                    .get_tags_by_name_and_type(&tags)?
                    .try_to_domain()
            })
    }

    fn get_image_tags_of_image(&self, image_id: &ImageId) -> Result<Vec<ImageTag>, anyhow::Error> {
        debug!("Getting image tags for image: {:?}", image_id);
        self.get_connection()?
            .transaction(|conn| conn.tag_dao().get_all_image_tags_by_image(**image_id))?
            .try_to_domain()
    }

    fn add_image_tags_to_image_by_user(&self, tags: Vec<Tag>, image_id: ImageId, user: User) -> Result<usize, anyhow::Error> {
        debug!("Adding image tags to image {:?}: {:?}", image_id, tags);
        Ok(self.get_connection()?
            .transaction(|conn| -> Result<usize, anyhow::Error> {
                let image_tag_insert_rows = to_user_image_tag_insert_rows(&tags, image_id, &user);
                let created_image_tag_rows = conn
                    .tag_dao()
                    .insert_image_tags(&image_tag_insert_rows)?;

                Ok(created_image_tag_rows.len())
            })?)
    }
}

fn to_user_image_tag_insert_rows(tags: &[Tag], image_id: ImageId, user: &User) -> Vec<ImageTagInsertRow> {
    tags
        .iter()
        .map(|tag|
                 ImageTagInsertRow {
                     image_id: *image_id,
                     tag_id: *tag.id,
                     user_id: Some(user.id),
                     source_site: None,
                 },
        )
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dao::test::{insert_test_image, insert_test_tag, insert_test_tag_with, insert_test_user};
    use crate::models::TagType;
    use crate::test::test_db;
    use assertables::{assert_bag_eq, assert_len_eq_x};
    use kani_domain_api_model::tag::TagName;
    use std::vec;

    #[test]
    #[serial_test::serial]
    fn test_get_all_tags() {
        let db = test_db();
        let mut connection = db.get_connection().unwrap();
        insert_test_tag(&mut connection).unwrap();

        let tags = db.get_all_tags().unwrap();

        assert_len_eq_x!(&tags, 1);
    }

    mod test_get_tags_create_if_missing {
        use super::*;

        #[test]
        #[serial_test::serial]
        fn should_return_all_requested_tags() {
            let db = test_db();
            let mut connection = db.get_connection().unwrap();
            let new_tag = NewTag {
                tag_name:  TagName::try_from("newTag".to_owned()).unwrap(),
                tag_type: kani_domain_api_model::tag::TagType::Character,
                ..NewTag::stub()
            };
            let existing_tag = NewTag {
                tag_name:  TagName::try_from("existingTag".to_owned()).unwrap(),
                tag_type: kani_domain_api_model::tag::TagType::General,
                ..NewTag::stub()
            };
            insert_test_tag_with(&mut connection, TagType::from_domain(existing_tag.tag_type.clone()), (*existing_tag.tag_name).to_owned()).unwrap();
            let tags = vec!(new_tag.clone(), existing_tag.clone());

            let result = db.get_tags_create_if_missing(tags).unwrap();

            let result_tags: Vec<(TagName, kani_domain_api_model::tag::TagType)> = result.into_iter()
                .map(|tag| (tag.tag_name, tag.tag_type.clone()))
                .collect();
            assert_bag_eq!(
                result_tags,
                vec!(
                    (new_tag.tag_name, new_tag.tag_type),
                    (existing_tag.tag_name, existing_tag.tag_type)
                ),
            );
        }

        #[test]
        #[serial_test::serial]
        fn should_create_all_tags_if_none_exist() {
            let db = test_db();
            let new_tag1 = NewTag {
                tag_name:  TagName::try_from("newTag1".to_owned()).unwrap(),
                tag_type: kani_domain_api_model::tag::TagType::Character,
                ..NewTag::stub()
            };
            let new_tag2 = NewTag {
                tag_name:  TagName::try_from("newTag2".to_owned()).unwrap(),
                tag_type: kani_domain_api_model::tag::TagType::General,
                ..NewTag::stub()
            };
            let new_tags = vec!(new_tag1.clone(), new_tag2.clone());

            db.get_tags_create_if_missing(new_tags).unwrap();
            let result = db.get_all_tags().unwrap();

            let result_tags: Vec<(TagName, kani_domain_api_model::tag::TagType)> = result.into_iter()
                .map(|tag| (tag.tag_name, tag.tag_type.clone()))
                .collect();
            assert_bag_eq!(
                result_tags,
                vec!(
                    (new_tag1.tag_name, new_tag1.tag_type),
                    (new_tag2.tag_name, new_tag2.tag_type)
                ),
            );
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_add_and_get_image_tags_to_image() {
        let db = test_db();
        let mut connection = db.get_connection().unwrap();
        let user = insert_test_user(&mut connection).unwrap();
        let db_image = insert_test_image(&mut connection).unwrap();
        let image_id = ImageId(db_image.id);
        let new_tag = NewTag {
            tag_name:  TagName::try_from("newTag".to_owned()).unwrap(),
            tag_type: kani_domain_api_model::tag::TagType::Character,
            ..NewTag::stub()
        };
        let tag = insert_test_tag_with(&mut connection, TagType::from_domain(new_tag.tag_type), (*new_tag.tag_name).to_owned()).unwrap()
            .try_to_domain().unwrap();

        db.add_image_tags_to_image_by_user(vec![tag.clone()], image_id.clone(), user.into()).unwrap();
        let result = db.get_image_tags_of_image(&image_id).unwrap();

        assert_len_eq_x!(&result, 1);
        assert_eq!(result.first().unwrap().tag, tag);
    }

}
