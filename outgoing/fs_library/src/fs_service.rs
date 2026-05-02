use std::path::PathBuf;

use async_trait::async_trait;
use bytes::Bytes;
use pantsu_domain::common::result::Result;
use pantsu_domain::image::PantsuImage;
use pantsu_domain::library::{Library, LibraryService};

use crate::library::PantsuLibrary;

pub struct DefaultFsService {
    lib_path: PathBuf,
}

impl DefaultFsService {
    pub fn new(lib_path: PathBuf) -> Self {
        return DefaultFsService { lib_path };
    }
}

#[async_trait]
impl LibraryService for DefaultFsService {
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
