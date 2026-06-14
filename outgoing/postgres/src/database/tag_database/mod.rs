mod converter;

use crate::dao::Dao;
use crate::database::converter::{FromDomain, TryToDomain};
use crate::models::image::ImageRow;
use crate::models::image_tag::ImageTagInsertRow;
use crate::models::tag::{TagInsertRow, TagRow};
use crate::Postgres;
use diesel::{Connection, PgConnection};
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::tag::image_tag::ImageTag;
use kani_domain_api_model::tag::{NewTag, Tag};
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::tag_database::{AddImageTagsByUserError, TagDatabase};
use tracing::debug;

impl TagDatabase for Postgres {
    fn get_all_tags(&self) -> Result<Vec<Tag>, anyhow::Error> {
        debug!("Getting all tags");
        self.get_connection()?
            .transaction(|conn| conn.tag_dao().get_all_tags())?
            .try_to_domain()
    }

    fn get_image_tags_of_image(&self, image_id: &ImageId) -> Result<Vec<ImageTag>, anyhow::Error> {
        debug!("Getting image tags for image: {:?}", image_id);
        self.get_connection()?
            .transaction(|conn| conn.tag_dao().get_all_image_tags_by_image(**image_id))?
            .try_to_domain()
    }

    fn add_image_tags_to_image_by_user(&self, new_tags: Vec<NewTag>, image_id: ImageId, user: User) -> Result<usize, AddImageTagsByUserError> {
        debug!("Adding image tags to image {:?}: {:?}", image_id, new_tags);
        let tags: Vec<TagInsertRow> = FromDomain::from_domain(new_tags);
        Ok(self.get_connection()?
            .transaction(|mut conn| -> Result<usize, anyhow::Error> {
                let image_row = conn.image_dao().get_image_by_id(*image_id)?
                    .ok_or_else(|| AddImageTagsByUserError::ImageNotFound(image_id))?;

                let tag_rows = get_tag_rows_insert_missing(&mut conn, tags)?;
                let image_tag_insert_rows = to_user_image_tag_insert_rows(&tag_rows, &image_row, &user);
                let created_image_tag_rows = conn
                    .tag_dao()
                    .insert_image_tags(&image_tag_insert_rows)?;

                Ok(created_image_tag_rows.len())
            })?)
    }
}

fn get_tag_rows_insert_missing(conn: &mut PgConnection, tags: Vec<TagInsertRow>) -> Result<Vec<TagRow>, anyhow::Error> {
    let created_tags = conn.tag_dao().insert_tags_if_missing(&tags)?;
    debug!("Created missing tags: {:?}", created_tags);
    conn.tag_dao().get_tags_by_name_and_type(&tags)
}

fn to_user_image_tag_insert_rows(tag_rows: &[TagRow], image_row: &ImageRow, user: &User) -> Vec<ImageTagInsertRow> {
    tag_rows
        .iter()
        .map(|tag|
                 ImageTagInsertRow {
                     image_id: image_row.id,
                     tag_id: tag.id,
                     user_id: Some(user.id),
                     source_site: None,
                 },
        )
        .collect()
}
