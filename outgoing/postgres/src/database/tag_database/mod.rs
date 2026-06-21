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

    fn add_image_tags_to_image_by_user(&self, new_tags: Vec<Tag>, image_id: ImageId, user: User) -> Result<usize, anyhow::Error> {
        debug!("Adding image tags to image {:?}: {:?}", image_id, new_tags);
        Ok(self.get_connection()?
            .transaction(|conn| -> Result<usize, anyhow::Error> {
                let image_tag_insert_rows = to_user_image_tag_insert_rows(&new_tags, image_id, &user);
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
