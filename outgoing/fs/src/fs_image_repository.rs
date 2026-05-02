use std::path::PathBuf;

use async_trait::async_trait;
use bytes::Bytes;
use pantsu_domain::api::outgoing::ImageRepository;
use pantsu_domain::common::result::Result;
use pantsu_domain::image::PantsuImage;
use pantsu_domain::library::Library;

use crate::library::PantsuLibrary;

pub struct FsImageRepository {
    lib_path: PathBuf,
}

impl FsImageRepository {
    pub fn new(lib_path: PathBuf) -> Self {
        return FsImageRepository { lib_path };
    }
}

#[async_trait]
impl ImageRepository for FsImageRepository {
    async fn store_image(&self, image: PantsuImage, file_content: Bytes) -> Result<()> {
        let library = PantsuLibrary::new(self.lib_path.clone()).await?;
        library.store_image(&image, file_content.clone()).await?;
        Ok(())
    }
    
    async fn store_jpg_thumbnail(
        &self,
        image: &PantsuImage,
        file_content: Bytes,
    ) -> Result<()> {
        let library = PantsuLibrary::new(self.lib_path.clone()).await?;
        library.create_thumbnail(&image, file_content).await?;
        Ok(())
    }
}
