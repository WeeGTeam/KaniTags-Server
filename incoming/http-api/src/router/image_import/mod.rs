use crate::auth_middleware::current_user;
use crate::router::AppState;
use async_trait::async_trait;
use axum::extract::Multipart;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use headers::Host;
use kani_domain::common::error::Error;
use kani_openapi::apis::image_import::{
    ImageImport, ImportImageResponse, StartImportSessionResponse,
};
use kani_openapi::models::{ImportImagePathParams, ImportSession};

#[async_trait]
impl ImageImport<Error> for AppState {
    async fn import_image(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &ImportImagePathParams,
        mut body: Multipart,
    ) -> Result<ImportImageResponse, Error> {
        let user = current_user();
        let field = body.next_field().await.unwrap().unwrap();
        let file_name = field.file_name().unwrap().to_owned();
        let file_data = field.bytes().await.unwrap();
        let import_session_id: i64 = path_params.id.parse().unwrap();

        self.image_management_service.import_image(&user, import_session_id, file_name, file_data).await.map_err(|e| Error::Unknown(e.to_string()))?;
        Ok(ImportImageResponse::Status201_Imported)
    }

    async fn start_import_session(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<StartImportSessionResponse, Error> {
        let user = current_user();

        let session = self.image_management_service.start_import_session(&user).await
            .map_err(|e| Error::Unknown(e.to_string()))?;
        Ok(StartImportSessionResponse::Status201_ImportSessionStarted(
            ImportSession {
                id: session.id.to_string(),
            },
        ))
    }
}
