use axum::extract::State;
use axum_typed_multipart::TryFromMultipart;
use bytes::Bytes;
use serde::Serialize;
use std::sync::Arc;
use tracing::log::{debug, info};
use utoipa::ToSchema;

use crate::common::result;
use crate::image::image_id::ImageId;
use crate::image::{image_id, PantsuImage};
use crate::routes::multipart::Multipart;
use crate::routes::AppState;
use crate::worker::fs::fs_service::FsService;

#[derive(TryFromMultipart, ToSchema, Debug)]
#[schema(rename_all = "camelCase")]
pub struct ImageImport {
    #[schema(value_type = String, format = Binary, content_media_type = "application/octet-stream")]
    #[form_data(limit = "unlimited")]
    pub image_file: Bytes,
    #[schema(value_type = String)]
    pub image_id: ImageId,
}

#[utoipa::path(
    post,
    path = "/import",
    responses(
        (status = OK, description = "Image import successful", body = String),
        (status = INTERNAL_SERVER_ERROR, description = "Image import failed", body = u32)
    ),
    request_body(content = ImageImport, content_type = "multipart/form-data")
)]
pub async fn import(State(state): State<AppState>, image_import: Multipart<ImageImport>) -> result::Result<()> {
    debug!("{:?}", image_import.image_id);
    import_impl(state.fs_service.clone(), image_import.data).await?;
    Ok(())
}

async fn import_impl(fs_service: Arc<dyn FsService + Sync + Send>, image_import: ImageImport) -> result::Result<()> {
    let image = PantsuImage::try_from(image_import.image_file.as_ref())?;
    image_id::verify_image_id(&image_import.image_id, image.id())?;

    // TODO: import: check if file exists (in db)

    info!("Store image in library: '{}'", image.filename());
    fs_service.store_image(image.clone(), image_import.image_file.clone()).await?;

    // TODO: add to db

    Ok(())
}
