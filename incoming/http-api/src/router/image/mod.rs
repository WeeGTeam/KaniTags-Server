use crate::auth_middleware::current_user;
use crate::converter::{FromDomain, TryToDomain};
use crate::error::HttpApiUnhandledError;
use crate::router::AppState;
use anyhow::Context;
use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use headers::Host;
use kani_domain_api_incoming::image_search_service::GetImageError;
use kani_domain_api_model::image_id::ImageId;
use kani_openapi::apis::image::Image;
use kani_openapi::apis::image::{GetImageDetailsResponse, GetImagesResponse};
use kani_openapi::models::{GetImageDetailsPathParams, GetImagesQueryParams, ImageDetailsDto};
use std::num::ParseIntError;

#[async_trait]
impl Image<HttpApiUnhandledError> for AppState {

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
        let images = self.image_search_service.search_images(&user, filter).await
            .context("Failed to search images")?;
        Ok(GetImagesResponse::Status200_Ok(Vec::from_domain(images)))
    }

    async fn get_image_details(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &GetImageDetailsPathParams
    ) -> Result<GetImageDetailsResponse, HttpApiUnhandledError> {
        let user = current_user();
        let image_id: i64 = path_params.id.parse().map_err(|e: ParseIntError| HttpApiUnhandledError::GenericBadRequest(e.into()))?;
        match self.image_search_service.get_image(&user, ImageId(image_id)).await {
            Ok(image) => Ok(GetImageDetailsResponse::Status200_Ok(ImageDetailsDto::from_domain(image))),
            Err(e @ GetImageError::ImageNotFound(_)) => Err(HttpApiUnhandledError::GenericNotFound(e.into())),
            Err(e @ GetImageError::Unknown(_)) => Err(HttpApiUnhandledError::Unknown(e.into()))
        }
    }
}



