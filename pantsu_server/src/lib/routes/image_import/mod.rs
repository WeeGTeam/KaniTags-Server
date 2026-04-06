use crate::common::error::Error;
use crate::common::result;
use crate::image::PantsuImage;
use crate::routes::AppState;
use crate::worker::fs::fs_service::FsService;
use async_trait::async_trait;
use axum::extract::Multipart;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use bytes::Bytes;
use headers::Host;
use pantsu_openapi::apis::image_import::{
    ImageImport, ImportImageResponse, StartImportSessionResponse,
};
use pantsu_openapi::models::{ImportImagePathParams, ImportSession};
use std::sync::Arc;
use tracing::info;

#[async_trait]
impl ImageImport<Error> for AppState {
    async fn import_image(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &ImportImagePathParams,
        mut body: Multipart,
    ) -> Result<ImportImageResponse, Error> {
        let field = body.next_field().await.unwrap().unwrap();
        let file_name = field.file_name().unwrap().to_owned();
        let file_data = field.bytes().await.unwrap();
        import_impl(self.fs_service.clone(), &file_name, file_data).await?;
        Ok(ImportImageResponse::Status201_Imported)
    }

    async fn start_import_session(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<StartImportSessionResponse, Error> {
        Ok(StartImportSessionResponse::Status201_ImportSessionStarted(
            ImportSession {
                id: "1234".to_owned(),
            },
        ))
    }
}

async fn import_impl(
    fs_service: Arc<dyn FsService + Sync + Send>,
    image_name: &str,
    image_data: Bytes,
) -> result::Result<()> {
    let image = PantsuImage::try_from(image_data.as_ref())?;
    // image_id::verify_image_id(&image_import.image_id, image.id())?;

    // TODO: import: check if file exists (in db)

    info!("Store image in library: '{}'", image.filename());
    fs_service.store_image(image.clone(), image_data).await?;

    // TODO: add to db

    Ok(())
}
