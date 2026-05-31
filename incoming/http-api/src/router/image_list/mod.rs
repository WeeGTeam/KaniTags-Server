use crate::auth_middleware::current_user;
use crate::converter::{FromDomain, TryToDomain};
use crate::error::HttpApiUnhandledError;
use crate::router::AppState;
use anyhow::Context;
use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use headers::Host;
use kani_openapi::apis::image_list::{GetImagesResponse, ImageList};
use kani_openapi::models::GetImagesQueryParams;

#[async_trait]
impl ImageList<HttpApiUnhandledError> for AppState {
    async fn get_images(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        query_params: &GetImagesQueryParams,
    ) -> Result<GetImagesResponse, HttpApiUnhandledError> {
        let user = current_user();
        let filter = query_params.try_to_domain()
            .context("Failed to convert query params to domain")
            .map_err(|e| HttpApiUnhandledError::GenericBadRequest(e))?;
        let images = self.image_search_service.search_images(&user, &filter)
            .context("Failed to search images")?;
        Ok(GetImagesResponse::Status200_Ok(Vec::from_domain(images)))
    }
}



