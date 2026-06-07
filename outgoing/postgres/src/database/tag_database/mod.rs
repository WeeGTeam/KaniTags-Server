mod converter;

use crate::dao::Dao;
use crate::database::converter::TryToDomain;
use crate::Postgres;
use diesel::Connection;
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
}
