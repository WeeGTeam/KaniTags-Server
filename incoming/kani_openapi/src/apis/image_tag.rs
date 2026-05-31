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
pub enum AddImageTagsResponse {
    /// tags added to image
    Status201_TagsAddedToImage
    (Vec<models::ImageTag>)
    ,
    /// Invalid tag or image id
    Status400_InvalidTagOrImageId
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetImageTagsResponse {
    /// Ok
    Status200_Ok
    (Vec<models::ImageTag>)
    ,
    /// image not found
    Status404_ImageNotFound
}




/// ImageTag
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ImageTag<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// AddImageTags - POST /image/{id}/tags
    async fn add_image_tags(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::AddImageTagsPathParams,
            body: &Vec<models::NewImageTag>,
    ) -> Result<AddImageTagsResponse, E>;

    /// GetImageTags - GET /image/{id}/tags
    async fn get_image_tags(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::GetImageTagsPathParams,
    ) -> Result<GetImageTagsResponse, E>;
}
