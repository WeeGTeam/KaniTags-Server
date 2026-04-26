use crate::router::AppState;
use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use headers::Host;
use pantsu_domain::common::error::Error;
use pantsu_openapi::apis::image_list::{GetImagesResponse, ImageList};
use pantsu_openapi::models::{GetImagesQueryParams, ImageInfo};

#[async_trait]
impl ImageList<Error> for AppState {
    async fn get_images(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _query_params: &GetImagesQueryParams,
    ) -> Result<GetImagesResponse, Error> {
        Ok(GetImagesResponse::Status200_Ok(vec![ImageInfo::new(
            "3b6368639f3e17fa".to_owned(),
        )]))
    }
}
