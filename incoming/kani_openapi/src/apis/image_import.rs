use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use bytes::Bytes;
use headers::Host;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetImportSessionsResponse {
    /// import sessions
    Status200_ImportSessions
    (Vec<models::ImportSession>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ImportImageResponse {
    /// imported
    Status201_Imported
    ,
    /// image already exists
    Status409_ImageAlreadyExists
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum StartImportSessionResponse {
    /// import session started
    Status201_ImportSessionStarted
    (String)
}




/// ImageImport
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ImageImport<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// GetImportSessions - GET /image/importSession
    async fn get_import_sessions(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<GetImportSessionsResponse, E>;

    /// ImportImage - POST /image/import/{id}
    async fn import_image(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::ImportImagePathParams,
    body: Multipart,
    ) -> Result<ImportImageResponse, E>;

    /// StartImportSession - POST /image/importSession
    async fn start_import_session(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<StartImportSessionResponse, E>;
}
