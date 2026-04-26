use crate::common::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait ReverseImageSearchService {
    async fn get_sauce(&self, image: String) -> Result<String, Error>;
}
