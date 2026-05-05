use crate::router::AppState;
use async_trait::async_trait;
use axum::extract::Multipart;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use headers::Host;
use pantsu_domain::common::error::Error;
use pantsu_openapi::apis::image_import::{
    ImageImport, ImportImageResponse, StartImportSessionResponse,
};
use pantsu_openapi::models::{ImportImagePathParams, ImportSession};

#[async_trait]
impl ImageImport<Error> for AppState {
    async fn import_image(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &ImportImagePathParams,
        mut body: Multipart,
    ) -> Result<ImportImageResponse, Error> {
        let field = body.next_field().await.unwrap().unwrap();
        let file_name = field.file_name().unwrap().to_owned();
        let file_data = field.bytes().await.unwrap();
        
        self.image_management_service.import_image(file_name, file_data).await.map_err(|e| Error::Unknown(e.to_string()))?;
        Ok(ImportImageResponse::Status201_Imported)
    }

    async fn start_import_session(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<StartImportSessionResponse, Error> {
        Ok(StartImportSessionResponse::Status201_ImportSessionStarted(
            ImportSession {
                id: "1234".to_owned(),
            },
        ))
    }
}
