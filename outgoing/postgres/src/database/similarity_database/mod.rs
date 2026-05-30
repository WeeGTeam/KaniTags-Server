pub mod converter;

use crate::dao::Dao;
use crate::Postgres;
use diesel::Connection;
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::similarity::{SimilarImage, SimilarImagePair};
use kani_domain_api_outgoing::database::similarity_database::SimilarityDatabase;
use tracing::debug;

impl SimilarityDatabase for Postgres {
    fn get_similar_images(&self, image_id: &ImageId) -> Result<Vec<SimilarImage>, anyhow::Error> {
        debug!("Getting similar images for image with id: {}", image_id);
        let mut connection = self.get_connection()?;
        let results = connection.transaction(|conn| conn.image_dao().get_similar_images_by_id_hash(&image_id.0, 30, 40))?;
        Ok(results.into_iter().map(Into::into).collect::<Vec<_>>())
    }

    fn get_all_similar_images(&self) -> Result<Vec<SimilarImagePair>, anyhow::Error> {
        debug!("Getting all similar images");
        let mut connection = self.get_connection()?;
        let results = connection.transaction(|conn| conn.image_dao().get_all_similar_images(30, 40))?;
        Ok(results.into_iter().map(Into::into).collect::<Vec<_>>())
    }
}
