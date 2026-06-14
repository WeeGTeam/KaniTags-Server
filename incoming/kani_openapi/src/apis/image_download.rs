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
pub enum GetImageResponse {
    /// Ok
    Status200_Ok
    {
        body: ByteArray,
        content_type: String,
        content_disposition:
        String
    }
    ,
    /// image not found
    Status404_ImageNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetThumbnailImageResponse {
    /// Ok
    Status200_Ok
    {
        body: ByteArray,
        content_type: String,
        content_disposition:
        String
    }
    ,
    /// image not found
    Status404_ImageNotFound
}




/// ImageDownload
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ImageDownload<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// GetImage - GET /image/{id}
    async fn get_image(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::GetImagePathParams,
    ) -> Result<GetImageResponse, E>;

    /// GetThumbnailImage - GET /image/thumbnail/{id}
    async fn get_thumbnail_image(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::GetThumbnailImagePathParams,
    ) -> Result<GetThumbnailImageResponse, E>;
}
