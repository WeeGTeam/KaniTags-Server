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
pub enum GetImagesResponse {
    /// Ok
    Status200_Ok
    (Vec<models::ImageInfo>)
}




/// ImageList
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ImageList<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// GetImages - GET /images
    async fn get_images(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::GetImagesQueryParams,
    ) -> Result<GetImagesResponse, E>;
}
