use std::io::Write;

use bytes::Bytes;
use image::codecs::jpeg::JpegEncoder;
use tokio::task::spawn_blocking;

use crate::{api::model::thumbnail::ThumbnailOptions, common::{error::Error, result::Result}, image::image_id::ImageId};


pub const GALLERY_THUMBNAIL_OPTIONS: ThumbnailOptions = ThumbnailOptions {
    max_size: 512,
    jpg_quality: 80,
};
const INITIAL_THUMBNAIL_BUFFER_SIZE: usize = usize::pow(2, 14);


pub async fn create_thumbnail_in_memory(
    image_id: ImageId,
    image_data: Bytes,
    options: ThumbnailOptions,
) -> Result<Bytes> {
    let result_buffer = Vec::with_capacity(INITIAL_THUMBNAIL_BUFFER_SIZE);
    create_thumbnail(result_buffer, image_id, image_data, options).await
}

async fn create_thumbnail(
    mut result_writer: impl Write + Into<Bytes> + Send + 'static,
    image_id: ImageId,
    image_data: Bytes,
    options: ThumbnailOptions,
) -> Result<Bytes>{
    spawn_blocking(move || {
        let loaded_image = image::load_from_memory(&image_data).map_err(|_| {
            Error::Unknown(format!("Failed to load image \"{}\" into memory", image_id))
        })?;
        let thumbnail = loaded_image.thumbnail(options.max_size, options.max_size);

        let encoder = JpegEncoder::new_with_quality(&mut result_writer, options.jpg_quality);
        thumbnail.write_with_encoder(encoder).map_err(|e| {
            Error::Unknown(format!("Failed to encode thumbnail \"{}\": {}", image_id, e.to_string()))
        })?;
        Ok(result_writer.into())
    })
    .await
    .map_err(|_| {
        Error::Unknown("Failed to join blocking thread generating the thumbnail".to_owned())
    })?
}