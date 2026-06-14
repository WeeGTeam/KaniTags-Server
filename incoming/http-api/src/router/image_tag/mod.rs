use crate::auth_middleware::current_user;
use crate::converter::{FromDomain, TryToDomain};
use crate::error::HttpApiUnhandledError;
use crate::router::AppState;
use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use headers::Host;
use kani_domain_api_model::image_id::ImageId;
use kani_openapi::apis::image_tag::{AddImageTagsResponse, GetImageTagsResponse, ImageTag};
use kani_openapi::models::{AddImageTagsPathParams, GetImageTagsPathParams, NewImageTag};
use std::num::ParseIntError;

#[async_trait]
impl ImageTag<HttpApiUnhandledError> for AppState {
    async fn add_image_tags(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &AddImageTagsPathParams,
        body: &Vec<NewImageTag>,
    ) -> Result<AddImageTagsResponse, HttpApiUnhandledError> {
        let user = current_user();
        let image_id: i64 = path_params.id.parse().map_err(|e: ParseIntError| HttpApiUnhandledError::GenericBadRequest(e.into()))?;

        let added_tags_result = match body.iter()
            .map(|new_tag| new_tag.try_to_domain())
            .collect() {
            Ok(new_tags) => self.tag_service.add_image_tags(ImageId(image_id), new_tags, user),
            Err(_) => return Ok(AddImageTagsResponse::Status400_InvalidTagOrImageId),
        };

        match added_tags_result {
            Ok(added_tags) => Ok(AddImageTagsResponse::Status201_AllTagsOfImage(
                FromDomain::from_domain(added_tags)
            )),
            Err(e) => Err(HttpApiUnhandledError::Unknown(e.into())),
        }
    }

    async fn get_image_tags(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &GetImageTagsPathParams,
    ) -> Result<GetImageTagsResponse, HttpApiUnhandledError> {
        let image_id: i64 = path_params.id.parse().map_err(|e: ParseIntError| HttpApiUnhandledError::GenericBadRequest(e.into()))?;

        let image_tags = self.tag_service.get_image_tags(ImageId(image_id))
            .map_err(|e| HttpApiUnhandledError::Unknown(e.into()))?;

        Ok(GetImageTagsResponse::Status200_Ok(
            FromDomain::from_domain(image_tags)
        ))
    }
}
