use crate::error::HttpApiUnhandledError;
use crate::router::AppState;
use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use chrono::Utc;
use headers::Host;
use kani_openapi::apis::image_tag::{AddImageTagsResponse, GetImageTagsResponse, ImageTag};
use kani_openapi::models;
use kani_openapi::models::{AddImageTagsPathParams, GetImageTagsPathParams, NewImageTag};

#[async_trait]
impl ImageTag<HttpApiUnhandledError> for AppState {
    async fn add_image_tags(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &AddImageTagsPathParams,
        _body: &Vec<NewImageTag>,
    ) -> Result<AddImageTagsResponse, HttpApiUnhandledError> {
        todo!()
    }

    async fn get_image_tags(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &GetImageTagsPathParams,
    ) -> Result<GetImageTagsResponse, HttpApiUnhandledError> {
        Ok(GetImageTagsResponse::Status200_Ok(vec![
            models::ImageTag {
                tag_id: "12345".to_owned(),
                created_by: Some("12345".to_owned()),
                created_at: Utc::now(),
            },
        ]))
    }
}
