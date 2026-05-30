use crate::error::HttpApiUnhandledError;
use crate::router::AppState;
use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::cookie::CookieJar;
use headers::Host;
use kani_domain_api_incoming::image_management::GetImageError;
use kani_domain_api_model::image_format::ImageFormat;
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::thumbnail::ThumbnailKind;
use kani_openapi::apis::image_download::{
    GetFullImageResponse, GetThumbnailImageResponse, ImageDownload,
};
use kani_openapi::models::{GetFullImagePathParams, GetThumbnailImagePathParams};
use kani_openapi::types::ByteArray;
use std::str::FromStr;
use tracing::info;

#[async_trait]
impl ImageDownload<HttpApiUnhandledError> for AppState {
    async fn get_full_image(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &GetFullImagePathParams,
    ) -> Result<GetFullImageResponse, HttpApiUnhandledError> {
        let id = match ImageId::from_str(&_path_params.id) {
            Ok(id) => id,
            Err(e) => {
                info!("Invalid image id: {}", e);
                return Ok(GetFullImageResponse::Status404_ImageNotFound);
            }
        };
        match self.image_management_service.get_image(id).await {
            Ok((bytes, format)) => Ok(GetFullImageResponse::Status200_Ok(
                ByteArray(bytes.to_vec()),
                to_image_content_type(format),
            )),
            Err(GetImageError::ImageNotFound(image_id)) => {
                info!("Image not found: {}", image_id);
                Ok(GetFullImageResponse::Status404_ImageNotFound)
            }
            Err(GetImageError::Unknown(e)) => Err(HttpApiUnhandledError::Unknown(e.into())),
        }
    }

    async fn get_thumbnail_image(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &GetThumbnailImagePathParams,
    ) -> Result<GetThumbnailImageResponse, HttpApiUnhandledError> {
        let id = match ImageId::from_str(&_path_params.id) {
            Ok(id) => id,
            Err(e) => {
                info!("Invalid image id: {}", e);
                return Ok(GetThumbnailImageResponse::Status404_ImageNotFound);
            }
        };

        match self.image_management_service.get_thumbnail(id, ThumbnailKind::Gallery).await {
            Ok((bytes, format)) => Ok(GetThumbnailImageResponse::Status200_Ok(
                ByteArray(bytes.to_vec()),
                to_image_content_type(format),
            )),
            Err(GetImageError::ImageNotFound(image_id)) => {
                info!("Image not found: {}", image_id);
                Ok(GetThumbnailImageResponse::Status404_ImageNotFound)
            }
            Err(GetImageError::Unknown(e)) => Err(HttpApiUnhandledError::Unknown(e.into())),
        }
    }
}

fn to_image_content_type(format: ImageFormat) -> String {
    match format {
        ImageFormat::PNG => "image/png".to_owned(),
        ImageFormat::JPG => "image/jpeg".to_owned(),
    }
}
