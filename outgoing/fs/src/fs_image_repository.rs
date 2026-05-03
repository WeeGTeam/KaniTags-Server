use std::path::{Path, PathBuf};

use async_trait::async_trait;
use bytes::Bytes;
use pantsu_domain::{api::model::thumbnail::ThumbnailOptions, image::image_format::ImageFormat};
use pantsu_domain::api::outgoing::image_repository::ImageRepository;
use pantsu_domain::common::error::Error;
use pantsu_domain::common::result::Result;
use pantsu_domain::image::PantsuImage;
use tokio::{fs::{DirBuilder, OpenOptions}, io::{self, AsyncWriteExt}};


pub struct FsImageRepository {
    lib_path: PathBuf,
}

impl FsImageRepository {
    pub fn new(library_path: PathBuf) -> Self {
        return FsImageRepository { lib_path: library_path };
    }

    async fn get_library_directory(&self) -> Result<&Path> {
        ensure_directory_exists(&self.lib_path).await?;
        Ok(self.lib_path.as_path())
    }

    async fn get_thumbnail_directory(&self, options: ThumbnailOptions) -> Result<PathBuf> {
        let common_thumbnails_dir = self.lib_path.join("thumbnails");
        let thumbnail_dir = common_thumbnails_dir.join(get_thumbnail_directory_name(&options));
        ensure_directory_exists(&thumbnail_dir).await?;
        Ok(thumbnail_dir)
    }
}

#[async_trait]
impl ImageRepository for FsImageRepository {
    async fn store_image(&self, image: PantsuImage, file_content: Bytes) -> Result<()> {
        let library_dir = self.get_library_directory().await?;
        let path = library_dir.join(image.filename());

        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .await
            .map_err(|err| match err.kind() {
                io::ErrorKind::AlreadyExists => {
                    Error::UnexpectedImageExists(image.id().to_string())
                }
                _ => Error::Unknown(err.to_string()),
            })?;
        Ok(
            file
                .write_all(&file_content)
                .await
                .map_err(|e| Error::Unknown(e.to_string()))?
        )
    }
    
    async fn store_jpg_thumbnail(
        &self,
        image: &PantsuImage,
        file_content: Bytes,
        options: ThumbnailOptions
    ) -> Result<()> {
        let thumbnail_dir = self.get_thumbnail_directory(options).await?;
        let path = thumbnail_dir.join(image.filename_with_custom_extension(ImageFormat::JPG));
        
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .await
            .map_err(|_| {
                Error::Unknown(format!(
                    "Failed to create thumbnail file for image \"{}\"",
                    image.id()
                ))
            })?;
        Ok(
            file
                .write_all(&file_content)
                .await
                .map_err(|e| Error::Unknown(e.to_string()))?
        )
    }
}

async fn ensure_directory_exists(directory: &Path) -> Result<()> {
    DirBuilder::new()
        .recursive(true)
        .mode(0o770)
        .create(directory)
        .await
        .map_err(|_| {
            Error::Unknown(format!(
                "Failed to create directory: {}", directory.to_string_lossy()
            ))
        })
}

fn get_thumbnail_directory_name(options: &ThumbnailOptions) -> String {
    format!("{}x{}", options.max_size, options.max_size)
}
