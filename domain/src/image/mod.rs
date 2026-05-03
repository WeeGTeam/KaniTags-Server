use crate::api::model::image_format::ImageFormat;
use crate::api::model::image_id::ImageId;
use crate::{api::model::image::PantsuImage, common::error::Error};
use crate::common::result::Result;
use chrono::Utc;
use image::GenericImageView;

pub mod hash;
pub mod thumbnail;

impl TryFrom<&[u8]> for PantsuImage {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        let image = image::load_from_memory(bytes).map_err(|_| Error::TodoError())?;
        let image_id_hash = hash::get_id_hash(bytes);
        let image_perceptual_hash = hash::get_perceptual_hash(&image);
        let image_dimensions = image.dimensions();
        let image_format = ImageFormat::try_from(image::guess_format(bytes)
            .map_err(|_| Error::TodoError())?)?;
        Ok(
            PantsuImage {
                id: ImageId::new(image_id_hash, image_perceptual_hash),
                format: image_format,
                _dimensions: image_dimensions,
                _date_added: Utc::now(),
            }
        )
    }
}
