use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use bytes::Bytes;
use headers::Host;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AddImagesToCollectionResponse {
    /// images added to collection
    Status200_ImagesAddedToCollection
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreateCollectionResponse {
    /// collection created
    Status201_CollectionCreated
    (models::CollectionDto)
    ,
    /// collection already exists
    Status409_CollectionAlreadyExists
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteCollectionResponse {
    /// collection deleted
    Status200_CollectionDeleted
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetCollectionsResponse {
    /// Collections
    Status200_Collections
    (Vec<models::CollectionDto>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum RemoveImagesFromCollectionResponse {
    /// images removed from collection
    Status200_ImagesRemovedFromCollection
}




/// Collection
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Collection<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// AddImagesToCollection - POST /collections/{id}/images
    async fn add_images_to_collection(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::AddImagesToCollectionPathParams,
            body: &Vec<models::ImageId>,
    ) -> Result<AddImagesToCollectionResponse, E>;

    /// CreateCollection - POST /collections
    async fn create_collection(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
            body: &String,
    ) -> Result<CreateCollectionResponse, E>;

    /// DeleteCollection - DELETE /collections/{id}
    async fn delete_collection(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::DeleteCollectionPathParams,
    ) -> Result<DeleteCollectionResponse, E>;

    /// GetCollections - GET /collections
    async fn get_collections(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<GetCollectionsResponse, E>;

    /// RemoveImagesFromCollection - DELETE /collections/{id}/images
    async fn remove_images_from_collection(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::RemoveImagesFromCollectionPathParams,
            body: &Vec<models::ImageId>,
    ) -> Result<RemoveImagesFromCollectionResponse, E>;
}
