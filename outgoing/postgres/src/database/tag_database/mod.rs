mod converter;

use crate::dao::Dao;
use crate::database::converter::{FromDomain, TryToDomain};
use crate::models::image::ImageRow;
use crate::models::image_tag::{ImageTagInsertRow, ImageTagRow};
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
        debug!("Getting image tags for image: {}", image_id);
        self.get_connection()?
            .transaction(|conn| conn.tag_dao().get_all_image_tags_by_image(image_id.as_ref()))?
            .try_to_domain()
    }

    fn add_image_tags_to_image_by_user(&self, new_tags: Vec<NewTag>, image_id: ImageId, user: User) -> Result<Vec<ImageTag>, AddImageTagsByUserError> {
        debug!("Adding image tags to image {}: {:?}", image_id, new_tags);
        let tags: Vec<TagInsertRow> = FromDomain::from_domain(new_tags);
        let (image_tag_rows, tag_rows) = self.get_connection()?
            .transaction(|mut conn| -> Result<(Vec<ImageTagRow>, Vec<TagRow>), anyhow::Error> {
                let image_row = conn.image_dao().get_image_by_id_hash(image_id.as_ref())?
                    .ok_or_else(|| AddImageTagsByUserError::ImageNotFound(image_id))?;

                let tag_rows = get_or_insert_tag_rows(&mut conn, tags)?;
                let image_tag_insert_rows = to_user_image_tag_insert_rows(&tag_rows, &image_row, &user);
                let created_image_tag_rows = conn
                    .tag_dao()
                    .insert_image_tags(&image_tag_insert_rows)?;

                Ok((created_image_tag_rows, tag_rows))
            })?;

        image_tag_rows.into_iter()
            .map(|image_tag_row| {
                Ok(
                    combine_image_tag_row_with_tag_row(image_tag_row, &tag_rows)?
                        .try_to_domain()?
                )
            })
            .collect::<Result<Vec<ImageTag>, AddImageTagsByUserError>>()
    }
}

fn get_or_insert_tag_rows(conn: &mut PgConnection, tags: Vec<TagInsertRow>) -> Result<Vec<TagRow>, anyhow::Error> {
    let existing_tags = conn.tag_dao().get_tags_by_name_and_type(&tags)?;
    let non_existing_tags = tags.into_iter()
        .filter(|tag| !existing_tags.iter()
            .any(|existing_tag| existing_tag.tag_type == tag.tag_type && existing_tag.tag_name == tag.tag_name))
        .collect::<Vec<TagInsertRow>>();
    let created_tags = conn.tag_dao().insert_tags(&non_existing_tags)?;

    let combined_tag_rows = existing_tags.into_iter()
        .chain(created_tags)
        .collect();

    Ok(combined_tag_rows)
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

fn combine_image_tag_row_with_tag_row(image_tag_row: ImageTagRow, tag_rows: &[TagRow]) -> Result<(ImageTagRow, TagRow), anyhow::Error> {
    let matching_tag_row = tag_rows.iter()
        .find(|tag_row| tag_row.id == image_tag_row.tag_id)
        .ok_or_else(|| anyhow::anyhow!("Failed to find matching tag row for image tag row: {:?}", image_tag_row))?
        .clone();

    Ok((
        image_tag_row,
        matching_tag_row,
    ))
}
