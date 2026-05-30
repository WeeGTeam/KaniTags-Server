use std::path::{Path, PathBuf};

use anyhow::Context;
use async_trait::async_trait;
use bytes::Bytes;
use kani_domain_api_model::image::PantsuImage;
use kani_domain_api_model::image_format::ImageFormat;
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::thumbnail::ThumbnailOptions;
use kani_domain_api_outgoing::image_repository::{ImageRepository, LoadImageError, StoreImageError};
use tokio::{fs, fs::{DirBuilder, OpenOptions}, io::{self, AsyncWriteExt}};

pub struct FsImageRepository {
    lib_path: PathBuf,
}

impl FsImageRepository {
    pub fn new(library_path: PathBuf) -> Self {
        FsImageRepository { lib_path: library_path }
    }

    async fn get_library_directory(&self) -> Result<&Path, anyhow::Error> {
        ensure_directory_exists(&self.lib_path).await?;
        Ok(self.lib_path.as_path())
    }

    async fn get_thumbnail_directory(&self, options: &ThumbnailOptions) -> Result<PathBuf, anyhow::Error> {
        let common_thumbnails_dir = self.lib_path.join("thumbnails");
        let thumbnail_dir = common_thumbnails_dir.join(get_thumbnail_directory_name(&options));
        ensure_directory_exists(&thumbnail_dir).await?;
        Ok(thumbnail_dir)
    }
}

#[async_trait]
impl ImageRepository for FsImageRepository {
    async fn store_image(
        &self,
        image_id: &ImageId,
        format: &ImageFormat,
        file_content: Bytes,
    ) -> Result<(), StoreImageError> {
        let library_dir = self.get_library_directory().await?;
        let path = library_dir.join(get_image_filename(&image_id, &format));

        write_image_to_new_file(&file_content, &path, &image_id).await
    }

    async fn store_jpg_thumbnail(
        &self,
        image_id: &ImageId,
        file_content: Bytes,
        options: ThumbnailOptions
    ) -> Result<(), StoreImageError> {
        let thumbnail_dir = self.get_thumbnail_directory(&options).await?;
        let path = thumbnail_dir.join(get_image_filename(&image_id, &ImageFormat::JPG));

        write_image_to_new_file(&file_content, &path, &image_id).await
    }

    async fn load_image(
        &self,
        image: &PantsuImage,
    ) -> Result<Bytes, LoadImageError> {
        let library_dir = self.get_library_directory().await?;
        let path = library_dir.join(get_image_filename(&image.image_id, &image.format));

        read_image(&path).await
    }

    async fn load_jpg_thumbnail(
        &self,
        image_id: &ImageId,
        options: &ThumbnailOptions,
    ) -> Result<Bytes, LoadImageError> {
        let thumbnail_dir = self.get_thumbnail_directory(options).await?;
        let path = thumbnail_dir.join(get_image_filename(image_id, &ImageFormat::JPG));

        read_image(&path).await
    }
}

async fn ensure_directory_exists(directory: &Path) -> Result<(), anyhow::Error> {
    DirBuilder::new()
        .recursive(true)
        .mode(0o770)
        .create(directory)
        .await
        .with_context(|| format!("Failed to create required directory: {}", directory.to_string_lossy()))
}

async fn write_image_to_new_file(file_content: &Bytes, path: &Path, image_id: &ImageId) -> Result<(), StoreImageError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .await
        .map_err(|e| match e.kind() {
            io::ErrorKind::AlreadyExists => {
                StoreImageError::ImageAlreadyExists(path.to_owned())
            }
            _ => StoreImageError::Unknown(e.into()),
        })?;
    Ok(
        file
            .write_all(&file_content)
            .await
            .with_context(|| format!("Failed to write into image into file: {}", image_id))?
    )
}

async fn read_image(path: &Path) -> Result<Bytes, LoadImageError> {
    fs::read(&path).await
        .map(|loaded_image| Bytes::from(loaded_image))
        .map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => {
                LoadImageError::ImageNotFound(path.to_owned())
            }
            _ => LoadImageError::Unknown(e.into())
        })
}

fn get_thumbnail_directory_name(options: &ThumbnailOptions) -> String {
    format!("{}x{}", options.max_size, options.max_size)
}

pub fn get_image_filename(id: &ImageId, format: &ImageFormat) -> String {
    format!("{}.{}", id.format_id_hash(), format.extension())
}
