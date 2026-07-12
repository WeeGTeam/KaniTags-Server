use crate::auth_middleware::current_user;
use crate::converter::parse::parse_id;
use crate::converter::{FromDomain, TryToDomain};
use crate::error::HttpApiUnhandledError;
use crate::router::AppState;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use headers::Host;
use kani_domain_api_incoming::collection_service::{AddImagesToCollectionError, CreateCollectionError, DeleteCollectionError, LoadCollectionsError, RemoveImagesFromCollectionError};
use kani_domain_api_model::collection::CollectionId;
use kani_openapi::apis::collection::{
    AddImagesToCollectionResponse, Collection, CreateCollectionResponse, DeleteCollectionResponse,
    GetCollectionsResponse, RemoveImagesFromCollectionResponse,
};
use kani_openapi::models;
use kani_openapi::models::{
    AddImagesToCollectionPathParams, DeleteCollectionPathParams, ImageId,
    RemoveImagesFromCollectionPathParams,
};

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
        let collection_id = parse_id(&path_params.id, CollectionId)?;
        let image_ids = body.try_to_domain().map_err(|e| HttpApiUnhandledError::GenericBadRequest(e.into()))?;
        match self.collection_service.add_images_to_collection(&user, collection_id, &image_ids) {
            Ok(()) => Ok(AddImagesToCollectionResponse::Status200_ImagesAddedToCollection),
            Err(e @ AddImagesToCollectionError::CollectionDoesNotExist(_)) => Err(HttpApiUnhandledError::GenericNotFound(e.into())),
            Err(e @ AddImagesToCollectionError::InsufficientImageAccess(_)) => Err(HttpApiUnhandledError::GenericForbidden(e.into())),
            Err(e @ AddImagesToCollectionError::Unknown(_)) => Err(HttpApiUnhandledError::GenericBadRequest(e.into())),
        }
    }

    async fn create_collection(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        body: &String,
    ) -> Result<CreateCollectionResponse, HttpApiUnhandledError> {
        let user = current_user();
        match self.collection_service.create_collection(&user, body) {
            Ok(collection) => Ok(CreateCollectionResponse::Status201_CollectionCreated(models::Collection::from_domain(collection))),
            Err(CreateCollectionError::CollectionAlreadyExists(_)) => Ok(CreateCollectionResponse::Status409_CollectionAlreadyExists),
            Err(CreateCollectionError::Unknown(error)) => Err(HttpApiUnhandledError::GenericBadRequest(error)),
        }
    }

    async fn delete_collection(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &DeleteCollectionPathParams,
    ) -> Result<DeleteCollectionResponse, HttpApiUnhandledError> {
        let user = current_user();
        let collection_id = parse_id(&path_params.id, CollectionId)?;
        match self.collection_service.delete_collection(&user, collection_id) {
            Ok(()) => Ok(DeleteCollectionResponse::Status200_CollectionDeleted),
            Err(e @ DeleteCollectionError::CollectionDoesNotExist(_)) => Err(HttpApiUnhandledError::GenericNotFound(e.into())),
            Err(DeleteCollectionError::Unknown(error)) => Err(HttpApiUnhandledError::GenericBadRequest(error)),
        }
    }

    async fn get_collections(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<GetCollectionsResponse, HttpApiUnhandledError> {
        let user = current_user();
        match self.collection_service.load_collections_by_user(&user) {
            Ok(collections) => Ok(GetCollectionsResponse::Status200_Collections(Vec::<models::Collection>::from_domain(collections))),
            Err(e @ LoadCollectionsError::Unknown(_)) => Err(HttpApiUnhandledError::GenericBadRequest(e.into())),
        }
    }

    async fn remove_images_from_collection(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        path_params: &RemoveImagesFromCollectionPathParams,
        body: &Vec<ImageId>,
    ) -> Result<RemoveImagesFromCollectionResponse, HttpApiUnhandledError> {
        let user = current_user();
        let collection_id = parse_id(&path_params.id, CollectionId)?;
        let image_ids = body.try_to_domain().map_err(|e| HttpApiUnhandledError::GenericBadRequest(e.into()))?;
        match self.collection_service.remove_images_from_collection(&user, collection_id, &image_ids) {
            Ok(()) => Ok(RemoveImagesFromCollectionResponse::Status200_ImagesRemovedFromCollection),
            Err(e @ RemoveImagesFromCollectionError::CollectionDoesNotExist(_)) => Err(HttpApiUnhandledError::GenericNotFound(e.into())),
            Err(e @ RemoveImagesFromCollectionError::Unknown(_)) => Err(HttpApiUnhandledError::GenericBadRequest(e.into())),
        }
    }
}
