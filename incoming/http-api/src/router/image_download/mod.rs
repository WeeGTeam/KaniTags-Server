use crate::router::AppState;
use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::cookie::CookieJar;
use headers::Host;
use kani_domain::common::error::Error;
use kani_domain_api_model::image_format::ImageFormat;
use kani_domain_api_model::image_id::ImageId;
use kani_openapi::apis::image_download::{
    GetFullImageResponse, GetThumbnailImageResponse, ImageDownload,
};
use kani_openapi::models::{GetFullImagePathParams, GetThumbnailImagePathParams};
use kani_openapi::types::ByteArray;
use std::str::FromStr;

#[async_trait]
impl ImageDownload<Error> for AppState {
    async fn get_full_image(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &GetFullImagePathParams,
    ) -> Result<GetFullImageResponse, Error> {
        let id = ImageId::from_str(&_path_params.id).map_err(|_| Error::TodoError())?;
        let (bytes, format) = self.image_management_service.get_image(id).await.map_err(|_| Error::TodoError())?;

        Ok(GetFullImageResponse::Status200_Ok(
            ByteArray(bytes.to_vec()),
            to_image_content_type(format),
        ))
    }

    async fn get_thumbnail_image(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &GetThumbnailImagePathParams,
    ) -> Result<GetThumbnailImageResponse, Error> {
        let bytes: Vec<u8> =
            tokio::fs::read("3b6368639f3e17fa-3803887ff7833837f03e43e43e21303b61fe.png").await
                .map_err(|_| Error::TodoError())?;
        Ok(GetThumbnailImageResponse::Status200_Ok(
            ByteArray(bytes),
            "image/png".to_owned(),
        ))
    }
}

fn to_image_content_type(format: ImageFormat) -> String {
    match format {
        ImageFormat::PNG => "image/png".to_owned(),
        ImageFormat::JPG => "image/jpeg".to_owned(),
    }
}
