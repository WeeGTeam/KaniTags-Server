use bytes::Bytes;
use image::codecs::jpeg::JpegEncoder;
use tokio::task::spawn_blocking;
use std::{io, path::PathBuf};
use tokio::fs::{DirBuilder, OpenOptions};
use tokio::io::AsyncWriteExt;

use crate::image::image_format::ImageFormat;
use crate::{common::error::Error, common::result::Result, config::ServerConfig, image::PantsuImage};

const GALLERY_THUMBNAIL_OPTIONS: ThumbnailOptions = ThumbnailOptions {
    max_size: 512,
    jpg_quality: 80,
};

pub struct PantsuLibrary {
    library_path: PathBuf,
    gallery_thumbnail_path: PathBuf,
}

impl PantsuLibrary {
    pub async fn new(config: &ServerConfig) -> Result<Self> {
        DirBuilder::new()
            .recursive(true)
            .mode(0o770)
            .create(&config.library_path)
            .await
            .map_err(|err| Error::LibraryDirectoryError(config.library_path.clone(), err))?;

        let thumbnails_path = config.library_path.join("thumbnails");
        let gallery_thumbnails_path = thumbnails_path.join(GALLERY_THUMBNAIL_OPTIONS.get_directory_name());
        DirBuilder::new()
            .recursive(true)
            .mode(0o770)
            .create(&gallery_thumbnails_path)
            .await
            .map_err(|_| Error::Unknown(format!("Failed to create thumbnails library directory: {}", gallery_thumbnails_path.to_string_lossy())))?;

        return Ok(PantsuLibrary {
            library_path: config.library_path.clone(),
            gallery_thumbnail_path: gallery_thumbnails_path,
        })
    }

    pub async fn store_image(&self, image: &PantsuImage, file_content: Bytes) -> Result<()> {
        let path = self.library_path.join(image.filename());
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .await
            .map_err(|err| match err.kind() {
                io::ErrorKind::AlreadyExists => Error::UnexpectedImageExists(image.id().clone()),
                _ => Error::IoError(err),
            })?;
        Ok(file.write_all(&file_content).await?)
    }

    pub async fn create_thumbnail(&self, image: &PantsuImage, file_content: Bytes) -> Result<()> {
        let path = self.gallery_thumbnail_path.join(image.filename_with_custom_extension(ImageFormat::JPG));
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .await
            .map_err(|_| Error::Unknown(format!("Failed to create thumbnail file for image \"{}\"", image.id())))?;
        let file_std = file.into_std().await;

        let image_id = image.id().clone();
        spawn_blocking(move || {
            let loaded_image = image::load_from_memory(&file_content).map_err(|_| Error::Unknown(format!("Failed to load image \"{}\" into memory", image_id)))?;
            let thumbnail = loaded_image.thumbnail(GALLERY_THUMBNAIL_OPTIONS.max_size, GALLERY_THUMBNAIL_OPTIONS.max_size);

            let encoder = JpegEncoder::new_with_quality(file_std, GALLERY_THUMBNAIL_OPTIONS.jpg_quality);
            thumbnail.write_with_encoder(encoder).map_err(|_| Error::Unknown(format!("Failed to encode and write thumbnail \"{}\"", image_id)))
        }).await
            .map_err(|_| Error::Unknown("Failed to join blocking thread generating the thumbnail".to_owned()))?
    }

}

struct ThumbnailOptions {
    max_size: u32,
    jpg_quality: u8,
}

impl ThumbnailOptions {
    fn get_directory_name(&self) -> String {
        format!("{}x{}", self.max_size, self.max_size)
    }
}