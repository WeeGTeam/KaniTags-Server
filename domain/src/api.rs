pub mod incoming {
    use async_trait::async_trait;
    use bytes::Bytes;
    
    use crate::common::error::Error;
    
    
    #[async_trait]
    pub trait ImageManagementService {
        async fn import_image(&self, image_name: String, image_data: Bytes) -> Result<(), Error>;
    }
}

pub mod outgoing {
    use async_trait::async_trait;
    use bytes::Bytes;
    
    use crate::common::result::Result;
    use crate::image::PantsuImage;
    
    
    #[async_trait]
    pub trait ImageRepository {
        async fn store_image(&self, image: PantsuImage, file_content: Bytes) -> Result<()>;
        
        async fn store_jpg_thumbnail(
            &self,
            image: &PantsuImage,
            file_content: Bytes,
        ) -> Result<()>;
    }
}