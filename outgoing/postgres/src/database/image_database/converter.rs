use crate::models::image::{ImageInsertRow, ImageRow};
use crate::models::import_session::ImportSessionRow;
use kani_domain_api_model::image::{CreatePantsuImage, PantsuImage};
use kani_domain_api_model::image_format::ImageFormat;
use kani_domain_api_model::image_hash::IdHash;
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::import::{ImportSession, ImportSessionId};
use pgvector::Bit;

impl TryFrom<ImageRow> for PantsuImage {
    type Error = anyhow::Error;

    fn try_from(value: ImageRow) -> Result<Self, Self::Error> {
        let id_hash: IdHash = value.id_hash.try_into().map_err(|v: Vec<u8>| anyhow::anyhow!("invalid id hash of size {}", v.len()))?;
        Ok(PantsuImage {
            image_id: ImageId(id_hash),
            upload_filename: value.file_name,
            format: value.image_format.into(),
            dimensions: (value.res_width as u32, value.res_height as u32),
            date_added: Default::default(),
        })
    }
}

impl TryFrom<ImageRow> for ImageId {
    type Error = anyhow::Error;

    fn try_from(value: ImageRow) -> Result<Self, Self::Error> {
        let id_hash: IdHash = value.id_hash.try_into().map_err(|v: Vec<u8>| anyhow::anyhow!("invalid id hash of size {}", v.len()))?;
        Ok(ImageId(id_hash))
    }
}

impl From<&CreatePantsuImage> for ImageInsertRow {
    fn from(value: &CreatePantsuImage) -> Self {
        ImageInsertRow {
            id_hash: value.id_hash.to_vec(),
            perceptual_hash: Bit::from_bytes(&value.perceptual_hash),
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

impl Into<ImportSessionId> for ImportSessionRow {
    fn into(self) -> ImportSessionId {
        ImportSessionId(self.id)
    }
}

impl Into<ImportSession> for ImportSessionRow {
    fn into(self) -> ImportSession {
        ImportSession {
            id: self.id,
            user_id: self.user_id,
        }
    }
}
