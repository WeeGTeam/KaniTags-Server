use crate::router::AppState;
use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use chrono::Utc;
use headers::Host;
use pantsu_domain::common::error::Error;
use pantsu_openapi::apis::image_tag::{GetImageTagsResponse, ImageTag};
use pantsu_openapi::models;
use pantsu_openapi::models::GetImageTagsPathParams;

#[async_trait]
impl ImageTag<Error> for AppState {
    async fn get_image_tags(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &GetImageTagsPathParams,
    ) -> Result<GetImageTagsResponse, Error> {
        Ok(GetImageTagsResponse::Status200_Ok(vec![
            models::ImageTag::new("12345".to_owned(), "12345".to_owned(), Utc::now()),
        ]))
    }
}
