mod converter;

use crate::dao::Dao;
use crate::database::converter::TryToDomain;
use crate::Postgres;
use anyhow::Error;
use diesel::Connection;
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::tag::image_tag::ImageTag;
use kani_domain_api_model::tag::Tag;
use kani_domain_api_outgoing::database::tag_database::TagDatabase;
use tracing::debug;

impl TagDatabase for Postgres {
    fn get_all_tags(&self) -> Result<Vec<Tag>, anyhow::Error> {
        debug!("Getting all tags");
        self.get_connection()?
            .transaction(|conn| conn.tag_dao().get_all_tags())?
            .try_to_domain()
    }

    fn get_image_tags_of_image(&self, image_id: &ImageId) -> Result<Vec<ImageTag>, Error> {
        debug!("Getting image tags for image: {}", image_id);
        self.get_connection()?
            .transaction(|conn| conn.tag_dao().get_all_image_tags_by_image(image_id.as_ref()))?
            .try_to_domain()
    }
}
