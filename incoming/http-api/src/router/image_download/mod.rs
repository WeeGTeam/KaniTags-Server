use crate::router::AppState;
use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::cookie::CookieJar;
use headers::Host;
use pantsu_domain::common::error::Error;
use pantsu_openapi::apis::image_download::{
    GetFullImageResponse, GetThumbnailImageResponse, ImageDownload,
};
use pantsu_openapi::models::{GetFullImagePathParams, GetThumbnailImagePathParams};
use pantsu_openapi::types::ByteArray;
use tracing::info;

#[async_trait]
impl ImageDownload<Error> for AppState {
    async fn get_full_image(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &GetFullImagePathParams,
    ) -> Result<GetFullImageResponse, Error> {
        info!("loading image 3b6368639f3e17fa-3803887ff7833837f03e43e43e21303b61fe.png");
        let bytes: Vec<u8> =
            tokio::fs::read("3b6368639f3e17fa-3803887ff7833837f03e43e43e21303b61fe.png").await
                .map_err(|_| Error::TodoError())?;
        Ok(GetFullImageResponse::Status200_Ok(
            ByteArray(bytes),
            "image/png".to_owned(),
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
