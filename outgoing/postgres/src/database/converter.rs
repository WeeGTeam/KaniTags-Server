use crate::models::image::{ImageInsertRow, ImageRow};
use pantsu_domain::api::model::image::{CreatePantsuImage, PantsuImage};
use pantsu_domain::api::model::image_format::ImageFormat;
use pantsu_domain::api::model::image_hash::{IdHash, PerceptualHash};
use pantsu_domain::api::model::image_id::ImageId;
impl TryFrom<ImageRow> for PantsuImage {
    type Error = anyhow::Error;

    fn try_from(value: ImageRow) -> Result<Self, Self::Error> {
        let id_hash: IdHash = value.id_hash.try_into().map_err(|v: Vec<u8>| anyhow::anyhow!("invalid id hash of size {}", v.len()))?;
        let perceptual_hash: PerceptualHash = value.perceptual_hash.try_into().map_err(|v: Vec<u8>| anyhow::anyhow!("invalid perceptual hash of size {}", v.len()))?;
        Ok(PantsuImage {
            id: value.id,
            image_id: ImageId::new(id_hash, perceptual_hash),
            upload_filename: value.file_name,
            format: value.image_format.into(),
            dimensions: (value.res_width as u32, value.res_height as u32),
            date_added: Default::default(),
        })
    }
}

impl From<&CreatePantsuImage> for ImageInsertRow {
    fn from(value: &CreatePantsuImage) -> Self {
        ImageInsertRow {
            id_hash: value.id.get_id_hash().to_vec(),
            perceptual_hash: value.id.get_perceptual_hash().to_vec(),
            file_name: value.upload_filename.to_string(),
            image_format: (&value.format).into(),
            res_width: value.dimensions.0 as i32,
            res_height: value.dimensions.1 as i32,
        }
    }
}

impl From<crate::models::ImageFormat> for ImageFormat {
    fn from(value: crate::models::ImageFormat) -> Self {
        match value {
            crate::models::ImageFormat::PNG => ImageFormat::PNG,
            crate::models::ImageFormat::JPG => ImageFormat::JPG,
        }
    }
}

impl From<&ImageFormat> for crate::models::ImageFormat {
    fn from(value: &ImageFormat) -> Self {
        match value {
            ImageFormat::PNG => crate::models::ImageFormat::PNG,
            ImageFormat::JPG => crate::models::ImageFormat::JPG,
        }
    }
}
