use crate::auth_middleware::current_user;
use crate::converter::TryToDomain;
use crate::error::HttpApiUnhandledError;
use crate::router::AppState;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use headers::Host;
use kani_openapi::apis::collection::{
    AddImagesToCollectionResponse, Collection, CreateCollectionResponse, DeleteCollectionResponse,
    GetCollectionsResponse, RemoveImagesFromCollectionResponse,
};
use kani_openapi::models::{
    AddImagesToCollectionPathParams, DeleteCollectionPathParams, ImageId,
    RemoveImagesFromCollectionPathParams,
};
use std::num::ParseIntError;

#[async_trait::async_trait]
impl Collection<HttpApiUnhandledError> for AppState {
    async fn add_images_to_collection(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &AddImagesToCollectionPathParams,
        body: &Vec<ImageId>,
    ) -> Result<AddImagesToCollectionResponse, HttpApiUnhandledError> {
        let user = current_user();
        let collection_id = path_params.id.parse::<i64>().map_err(|e: ParseIntError| HttpApiUnhandledError::GenericBadRequest(e.into()))?;
        let image_ids = body.try_to_domain().map_err(|e| HttpApiUnhandledError::GenericBadRequest(e.into()))?;
        todo!()
    }

    async fn create_collection(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<CreateCollectionResponse, HttpApiUnhandledError> {
        todo!()
    }

    async fn delete_collection(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &DeleteCollectionPathParams,
    ) -> Result<DeleteCollectionResponse, HttpApiUnhandledError> {
        todo!()
    }

    async fn get_collections(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetCollectionsResponse, HttpApiUnhandledError> {
        todo!()
    }

    async fn remove_images_from_collection(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &RemoveImagesFromCollectionPathParams,
        body: &Vec<ImageId>,
    ) -> Result<RemoveImagesFromCollectionResponse, HttpApiUnhandledError> {
        todo!()
    }
}
