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
pub enum GetTagsResponse {
    /// Ok
    Status200_Ok
    (Vec<models::Tag>)
}




/// Tag
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Tag<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// GetTags - GET /tags
    async fn get_tags(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<GetTagsResponse, E>;
}
