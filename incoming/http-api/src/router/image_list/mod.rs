use crate::error::HttpApiUnhandledError;
use crate::router::AppState;
use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use headers::Host;
use kani_openapi::apis::image_list::{GetImagesResponse, ImageList};
use kani_openapi::models::{GetImagesQueryParams, ImageId};

#[async_trait]
impl ImageList<HttpApiUnhandledError> for AppState {
    async fn get_images(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _query_params: &GetImagesQueryParams,
    ) -> Result<GetImagesResponse, HttpApiUnhandledError> {
        Ok(GetImagesResponse::Status200_Ok(vec![
            ImageId("3b6368639f3e17fa".to_owned()),
        ]))
    }
}
