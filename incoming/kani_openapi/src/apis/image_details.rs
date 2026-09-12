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
pub enum GetImageDetailsResponse {
    /// Ok
    Status200_Ok
    (models::ImageDetailsDto)
    ,
    /// image not found
    Status404_ImageNotFound
}




/// ImageDetails
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ImageDetails<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// GetImageDetails - GET /image/{id}/details
    async fn get_image_details(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::GetImageDetailsPathParams,
    ) -> Result<GetImageDetailsResponse, E>;
}
