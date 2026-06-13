use crate::dao::Dao;
use crate::Postgres;
use diesel::Connection;
use kani_domain_api_model::image_id::ImageIdHash;
use kani_domain_api_model::similarity::{SimilarImage, SimilarImagePair};
use kani_domain_api_outgoing::database::similarity_database::SimilarityDatabase;
use tracing::debug;

impl SimilarityDatabase for Postgres {
    fn get_similar_images(&self, image_id_hash: &ImageIdHash) -> Result<Vec<SimilarImage>, anyhow::Error> {
        debug!("Getting similar images for image with id hash: {}", image_id_hash);
        let mut connection = self.get_connection()?;
        let results = connection.transaction(|conn| conn.image_dao().get_similar_images_by_id_hash(&image_id_hash.0, 30, 40))?;
        Ok(results.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?)
    }

    fn get_all_similar_images(&self) -> Result<Vec<SimilarImagePair>, anyhow::Error> {
        debug!("Getting all similar images");
        let mut connection = self.get_connection()?;
        let results = connection.transaction(|conn| conn.image_dao().get_all_similar_images(30, 40))?;
        Ok(results.into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>()?)
    }
}
