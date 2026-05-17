use blockhash::Image;

use crate::api::incoming::image_management::ImportImageError;
use kani_domain_api_model::{image::CreatePantsuImage, image_format::ImageFormat, image_id::ImageId};

pub mod hash;
pub mod thumbnail;


pub fn try_create_pantsu_image(filename: &str, bytes: &[u8]) -> Result<CreatePantsuImage, ImportImageError> {
    let image = image::load_from_memory(bytes).map_err(|e| ImportImageError::InvalidImage(e))?;
    let image_id_hash = hash::get_id_hash(bytes);
    let image_perceptual_hash = hash::get_perceptual_hash(&image);
    let image_dimensions = image.dimensions();
    let image_format = ImageFormat::try_from(
        image::guess_format(bytes).map_err(|_| ImportImageError::UnsupportedImageFormat(None))?
    )?;
    Ok(
        CreatePantsuImage {
            id: ImageId::new(image_id_hash, image_perceptual_hash),
            upload_filename: filename.to_string(),
            format: image_format,
            dimensions: image_dimensions,
        }
    )
}
