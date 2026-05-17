use std::io::Write;

use anyhow::Context;
use bytes::Bytes;
use image::codecs::jpeg::JpegEncoder;
use tokio::task::spawn_blocking;

use kani_domain_api_model::{image_id::ImageId, thumbnail::ThumbnailOptions};


pub const GALLERY_THUMBNAIL_OPTIONS: ThumbnailOptions = ThumbnailOptions {
    max_size: 512,
    jpg_quality: 80,
};
const INITIAL_THUMBNAIL_BUFFER_SIZE: usize = usize::pow(2, 14);


pub async fn create_thumbnail_in_memory(
    image_id: ImageId,
    image_data: Bytes,
    options: ThumbnailOptions,
) -> Result<Bytes, anyhow::Error> {
    let result_buffer = Vec::with_capacity(INITIAL_THUMBNAIL_BUFFER_SIZE);
    create_thumbnail(result_buffer, image_id, image_data, options).await
}

async fn create_thumbnail(
    mut result_writer: impl Write + Into<Bytes> + Send + 'static,
    image_id: ImageId,
    image_data: Bytes,
    options: ThumbnailOptions,
) -> Result<Bytes, anyhow::Error>{
    spawn_blocking(move || {
        let loaded_image = image::load_from_memory(&image_data)
            .with_context(|| format!("Failed to load image \"{}\" into memory", image_id))?;
        let thumbnail = loaded_image.thumbnail(options.max_size, options.max_size);

        let encoder = JpegEncoder::new_with_quality(&mut result_writer, options.jpg_quality);
        thumbnail.write_with_encoder(encoder)
            .with_context(|| format!("Failed to encode thumbnail \"{}\"", image_id))?;
        Ok(result_writer.into())
    })
    .await
    .context("Failed to join blocking thread generating the thumbnail")?
}
