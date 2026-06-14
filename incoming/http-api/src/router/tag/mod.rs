use crate::converter::FromDomain;
use crate::error::HttpApiUnhandledError;
use crate::router::AppState;
use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use headers::Host;
use kani_openapi::apis::tag::{GetTagsResponse, Tag};

#[async_trait]
impl Tag<HttpApiUnhandledError> for AppState {
    async fn get_tags(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<GetTagsResponse, HttpApiUnhandledError> {
        let tags = self.tag_service.get_tags()
            .map_err(|e| HttpApiUnhandledError::Unknown(e.into()))?;


        Ok(GetTagsResponse::Status200_Ok(
            FromDomain::from_domain(tags)
        ))
    }
}
