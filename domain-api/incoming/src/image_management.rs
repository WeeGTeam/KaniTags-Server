use async_trait::async_trait;
use bytes::Bytes;
use kani_domain_api_model::image::ImageDownloadData;
use kani_domain_api_model::image_format::ImageFormatError;
use kani_domain_api_model::image_id::{ImageId, ImageIdHash};
use kani_domain_api_model::import::{ImportSession, ImportSessionId};
use kani_domain_api_model::thumbnail::ThumbnailKind;
use kani_domain_api_model::user::User;
use thiserror::Error;

#[async_trait]
pub trait ImageManagementService {
    async fn import_image(&self, user: &User, import_session_id: ImportSessionId, image_name: String, image_data: Bytes) -> Result<(), ImportImageError>;

    async fn start_import_session(&self, user: &User) -> Result<ImportSessionId, StartImportSessionError>;

    async fn close_import_session(&self, user: &User, import_session_id: ImportSessionId) -> Result<(), CloseImportSessionError>;

    async fn get_import_sessions(&self, user: &User) -> Result<Vec<ImportSession>, GetImportSessionsError>;

    async fn get_image(&self, image_id: ImageId) -> Result<ImageDownloadData, GetImageError>;

    async fn get_thumbnail(&self, image_id: ImageId, kind: ThumbnailKind) -> Result<ImageDownloadData, GetImageError>;
}

#[derive(Error, Debug)]
pub enum ImportImageError {
    #[error("Image import internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),

    #[error("Import session does not exist: {0:?}")]
    MissingImportSession(ImportSessionId),

    #[error("Import session is closed: {0:?}")]
    ImportSessionClosed(ImportSessionId),

    #[error("Image has unsupported format: {0:?}")]
    UnsupportedImageFormat(Option<image::ImageFormat>),

    #[error("Unable to load Image: {0}")]
    InvalidImage(image::ImageError),

    #[error("Image already imported: IdHash({0})")]
    ImageAlreadyImported(ImageIdHash),
}

#[derive(Error, Debug)]
pub enum StartImportSessionError {
    #[error("Import session internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum CloseImportSessionError {
    #[error("Close import session internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),

    #[error("Import session does not exist: {0:?}")]
    ImportSessionMissing(ImportSessionId),

    #[error("Import session is already closed: {0:?}")]
    ImportSessionClosed(ImportSessionId),
}

#[derive(Error, Debug)]
pub enum GetImageError {
    #[error("Get image internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),

    #[error("Image not found: {0:?}")]
    ImageNotFound(ImageId),
}

#[derive(Error, Debug)]
pub enum GetImportSessionsError {
    #[error("Get import sessions internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
}


impl From<ImageFormatError> for ImportImageError {
    fn from(value: ImageFormatError) -> Self {
        match value {
            ImageFormatError::UnsupportedImageFormat(image_format) => ImportImageError::UnsupportedImageFormat(Some(image_format)),
        }
    }
}
