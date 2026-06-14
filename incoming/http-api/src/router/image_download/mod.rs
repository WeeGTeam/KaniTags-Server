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
use kani_openapi::apis::image_download::GetImageResponse;
use kani_openapi::apis::image_download::{
    GetThumbnailImageResponse, ImageDownload,
};
use kani_openapi::models::{GetImagePathParams, GetThumbnailImagePathParams};
use kani_openapi::types::ByteArray;
use std::num::ParseIntError;
use tracing::info;

#[async_trait]
impl ImageDownload<HttpApiUnhandledError> for AppState {
    async fn get_image(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &GetImagePathParams,
    ) -> Result<GetImageResponse, HttpApiUnhandledError> {
        let image_id: i64 = path_params.id.parse().map_err(|e: ParseIntError| HttpApiUnhandledError::GenericBadRequest(e.into()))?;

        match self.image_management_service.get_image(ImageId(image_id)).await {
            Ok((bytes, filename, format)) => Ok(GetImageResponse::Status200_Ok {
                body: ByteArray(bytes.to_vec()),
                content_type: to_image_content_type(format),
                content_disposition: format!("attachment; filename=\"{}\"", filename)
            }),
            Err(GetImageError::ImageNotFound(image_id)) => {
                info!("Image not found: {:?}", image_id);
                Ok(GetImageResponse::Status404_ImageNotFound)
            }
            Err(GetImageError::Unknown(e)) => Err(HttpApiUnhandledError::Unknown(e.into())),
        }
    }

    async fn get_thumbnail_image(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &GetThumbnailImagePathParams,
    ) -> Result<GetThumbnailImageResponse, HttpApiUnhandledError> {
        let image_id: i64 = path_params.id.parse().map_err(|e: ParseIntError| HttpApiUnhandledError::GenericBadRequest(e.into()))?;

        match self.image_management_service.get_thumbnail(ImageId(image_id), ThumbnailKind::Gallery).await {
            Ok((bytes, filename, format)) => Ok(GetThumbnailImageResponse::Status200_Ok {
                body: ByteArray(bytes.to_vec()),
                content_type: to_image_content_type(format),
                content_disposition: format!("attachment; filename=\"{}\"", filename)
            }),
            Err(GetImageError::ImageNotFound(image_id)) => {
                info!("Image not found: {:?}", image_id);
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
