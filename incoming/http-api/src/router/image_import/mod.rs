use crate::auth_middleware::current_user;
use crate::converter::FromDomain;
use crate::error::HttpApiUnhandledError;
use crate::router::AppState;
use async_trait::async_trait;
use axum::extract::Multipart;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use headers::Host;
use kani_domain_api_incoming::image_management::{CloseImportSessionError, ImportImageError};
use kani_domain_api_model::import::ImportSessionId;
use kani_openapi::apis::image_import::{CloseImportSessionResponse, GetImportSessionsResponse, ImageImport, ImportImageResponse, StartImportSessionResponse};
use kani_openapi::models::{CloseImportSessionPathParams, ImportImagePathParams, ImportSessionDto};
use std::num::ParseIntError;
use tracing::error;

#[async_trait]
impl ImageImport<HttpApiUnhandledError> for AppState {
    async fn import_image(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &ImportImagePathParams,
        mut body: Multipart,
    ) -> Result<ImportImageResponse, HttpApiUnhandledError> {
        let user = current_user();
        let field = body.next_field().await.unwrap().unwrap();
        let file_name = field.file_name().unwrap().to_owned();
        let file_data = field.bytes().await.unwrap();
        let import_session_id: i64 = path_params.id.parse().map_err(|e: ParseIntError| HttpApiUnhandledError::GenericBadRequest(e.into()))?;

        let result = self.image_management_service.import_image(&user, ImportSessionId(import_session_id), file_name, file_data).await;
        match result {
            Ok(_) => Ok(ImportImageResponse::Status201_Imported),
            Err(e @ ImportImageError::MissingImportSession(_)) => {
                error!("Failed to import image: {}", e);
                Ok(ImportImageResponse::Status404_ImportSessionMissing)
            },
            Err(e @ ImportImageError::ImportSessionClosed(_)) => {
                error!("Failed to import image: {}", e);
                Ok(ImportImageResponse::Status400_ImportSessionClosed)
            },
            Err(e) => Err(HttpApiUnhandledError::Unknown(e.into())),
        }
    }

    async fn start_import_session(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<StartImportSessionResponse, HttpApiUnhandledError> {
        let user = current_user();

        let session = self.image_management_service.start_import_session(&user).await
            .map_err(|e| HttpApiUnhandledError::Unknown(e.into()))?;
        Ok(StartImportSessionResponse::Status201_ImportSessionStarted(
            session.to_string(),
        ))
    }

    async fn close_import_session(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &CloseImportSessionPathParams
    ) -> Result<CloseImportSessionResponse, HttpApiUnhandledError> {
        let user = current_user();
        let import_session_id: i64 = path_params.id.parse()
            .map_err(|e: ParseIntError| HttpApiUnhandledError::GenericBadRequest(e.into()))?;

        let result = self.image_management_service.close_import_session(&user, ImportSessionId(import_session_id)).await;
        match result {
            Ok(_) => Ok(CloseImportSessionResponse::Status204_ImportSessionClosed),
            Err(CloseImportSessionError::ImportSessionMissing(_)) => Ok(CloseImportSessionResponse::Status404_ImportSessionMissing),
            Err(CloseImportSessionError::ImportSessionClosed(_)) => Ok(CloseImportSessionResponse::Status400_ImportSessionAlreadyClosed),
            Err(e)=> Err(HttpApiUnhandledError::Unknown(e.into())),
        }
    }

    async fn get_import_sessions(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<GetImportSessionsResponse, HttpApiUnhandledError> {
        let user = current_user();

        let sessions = self.image_management_service.get_import_sessions(&user).await
            .map_err(|e| HttpApiUnhandledError::Unknown(e.into()))?;
        let sessions = Vec::<ImportSessionDto>::from_domain(sessions);
        Ok(GetImportSessionsResponse::Status200_ImportSessions(sessions))
    }
}
