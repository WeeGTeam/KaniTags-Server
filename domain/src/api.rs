use async_trait::async_trait;
use bytes::Bytes;

use crate::common::error::Error;


#[async_trait]
pub trait ImageManagementService {
    async fn import_image(&self, image_name: String, image_data: Bytes) -> Result<(), Error>;
}