use pantsu_domain::common::{error::Error, result::Result};

use async_trait::async_trait;
use bytes::Bytes;
use pantsu_domain::image::image_format::ImageFormat;
use pantsu_domain::image::PantsuImage;
use pantsu_domain::library::{Library, ThumbnailOptions, GALLERY_THUMBNAIL_OPTIONS};
use std::{io, path::PathBuf};
use tokio::fs::{DirBuilder, OpenOptions};
use tokio::io::AsyncWriteExt;


pub struct PantsuLibrary {
    library_path: PathBuf,
    gallery_thumbnail_path: PathBuf,
}

impl PantsuLibrary {
    pub async fn new(library_path: PathBuf) -> Result<Self> {
        DirBuilder::new()
            .recursive(true)
            .mode(0o770)
            .create(&library_path)
            .await
            .map_err(|err| Error::LibraryDirectoryError(library_path.clone(), err))?;

        let thumbnails_path = library_path.join("thumbnails");
        let gallery_thumbnails_path =
            thumbnails_path.join(get_directory_name(&GALLERY_THUMBNAIL_OPTIONS));
        DirBuilder::new()
            .recursive(true)
            .mode(0o770)
            .create(&gallery_thumbnails_path)
            .await
            .map_err(|_| {
                Error::Unknown(format!(
                    "Failed to create thumbnails library directory: {}",
                    gallery_thumbnails_path.to_string_lossy()
                ))
            })?;

        return Ok(PantsuLibrary {
            library_path: library_path.clone(),
            gallery_thumbnail_path: gallery_thumbnails_path,
        });
    }
}

#[async_trait]
impl Library for PantsuLibrary {
    async fn store_image(&self, image: &PantsuImage, file_content: Bytes) -> Result<()> {
        let path = self.library_path.join(image.filename());
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
        Ok(file
            .write_all(&file_content)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?)
    }

    async fn create_thumbnail(
        &self,
        image: &PantsuImage,
        file_content: Bytes,
    ) -> Result<()> {
        let path = self
            .gallery_thumbnail_path
            .join(image.filename_with_custom_extension(ImageFormat::JPG));
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

fn get_directory_name(options: &ThumbnailOptions) -> String {
    format!("{}x{}", options.max_size, options.max_size)
}
