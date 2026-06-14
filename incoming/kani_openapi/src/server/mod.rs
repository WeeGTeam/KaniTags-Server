use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::{
    TypedHeader,
    extract::{CookieJar, Query as QueryExtra},
};
use bytes::Bytes;
use headers::Host;
use http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode, header::CONTENT_TYPE};
use tracing::error;
use validator::{Validate, ValidationErrors};

#[allow(unused_imports)]
use crate::{apis, models};
use crate::{header, types::*};
#[allow(unused_imports)]
use crate::{
    models::check_xss_map, models::check_xss_map_nested, models::check_xss_map_string,
    models::check_xss_string, models::check_xss_vec_string,
};


/// Setup API Server.
pub fn new<I, A, E>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::image_download::ImageDownload<E> + apis::image_import::ImageImport<E> + apis::image_list::ImageList<E> + apis::image_tag::ImageTag<E> + apis::tag::Tag<E> + Send + Sync + 'static,
    E: std::fmt::Debug + Send + Sync + 'static,
    
{
    // build our application with a route
    Router::new()
        .route("/image/import/{id}",
            post(import_image::<I, A, E>)
        )
        .route("/image/importSession",
            get(get_import_sessions::<I, A, E>).post(start_import_session::<I, A, E>)
        )
        .route("/image/importSession/{id}",
            delete(close_import_session::<I, A, E>)
        )
        .route("/image/thumbnail/{id}",
            get(get_thumbnail_image::<I, A, E>)
        )
        .route("/image/{id}",
            get(get_image::<I, A, E>)
        )
        .route("/image/{id}/tags",
            get(get_image_tags::<I, A, E>).post(add_image_tags::<I, A, E>)
        )
        .route("/images",
            get(get_images::<I, A, E>)
        )
        .route("/tags",
            get(get_tags::<I, A, E>)
        )
        .with_state(api_impl)
}


#[tracing::instrument(skip_all)]
fn get_image_validation(
  path_params: models::GetImagePathParams,
) -> std::result::Result<(
  models::GetImagePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// GetImage - GET /image/{id}
#[tracing::instrument(skip_all)]
async fn get_image<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::GetImagePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::image_download::ImageDownload<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_image_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



let result = api_impl.as_ref().get_image(
      
      &method,
      &host,
      &cookies,
        &path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::image_download::GetImageResponse::Status200_Ok
                                                    (body, content_type)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str(&content_type).unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")));
                                                  }

                                                  let body_content = body.0;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::image_download::GetImageResponse::Status404_ImageNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_thumbnail_image_validation(
  path_params: models::GetThumbnailImagePathParams,
) -> std::result::Result<(
  models::GetThumbnailImagePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// GetThumbnailImage - GET /image/thumbnail/{id}
#[tracing::instrument(skip_all)]
async fn get_thumbnail_image<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::GetThumbnailImagePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::image_download::ImageDownload<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_thumbnail_image_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



let result = api_impl.as_ref().get_thumbnail_image(
      
      &method,
      &host,
      &cookies,
        &path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::image_download::GetThumbnailImageResponse::Status200_Ok
                                                    (body, content_type)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str(&content_type).unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")));
                                                  }

                                                  let body_content = body.0;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::image_download::GetThumbnailImageResponse::Status404_ImageNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn close_import_session_validation(
  path_params: models::CloseImportSessionPathParams,
) -> std::result::Result<(
  models::CloseImportSessionPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// CloseImportSession - DELETE /image/importSession/{id}
#[tracing::instrument(skip_all)]
async fn close_import_session<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::CloseImportSessionPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::image_import::ImageImport<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    close_import_session_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



let result = api_impl.as_ref().close_import_session(
      
      &method,
      &host,
      &cookies,
        &path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::image_import::CloseImportSessionResponse::Status204_ImportSessionClosed
                                                => {
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::image_import::CloseImportSessionResponse::Status404_ImportSessionMissing
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::image_import::CloseImportSessionResponse::Status400_ImportSessionAlreadyClosed
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_import_sessions_validation(
) -> std::result::Result<(
), ValidationErrors>
{

Ok((
))
}
/// GetImportSessions - GET /image/importSession
#[tracing::instrument(skip_all)]
async fn get_import_sessions<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::image_import::ImageImport<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_import_sessions_validation(
    )
  ).await.unwrap();

  let Ok((
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



let result = api_impl.as_ref().get_import_sessions(
      
      &method,
      &host,
      &cookies,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::image_import::GetImportSessionsResponse::Status200_ImportSessions
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn import_image_validation(
  path_params: models::ImportImagePathParams,
) -> std::result::Result<(
  models::ImportImagePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// ImportImage - POST /image/import/{id}
#[tracing::instrument(skip_all)]
async fn import_image<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::ImportImagePathParams>,
 State(api_impl): State<I>,
  body: Multipart,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::image_import::ImageImport<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    import_image_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



let result = api_impl.as_ref().import_image(
      
      &method,
      &host,
      &cookies,
        &path_params,
          body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::image_import::ImportImageResponse::Status201_Imported
                                                => {
                                                  let mut response = response.status(201);
                                                  response.body(Body::empty())
                                                },
                                                apis::image_import::ImportImageResponse::Status409_ImageAlreadyExists
                                                => {
                                                  let mut response = response.status(409);
                                                  response.body(Body::empty())
                                                },
                                                apis::image_import::ImportImageResponse::Status404_ImportSessionMissing
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::image_import::ImportImageResponse::Status400_ImportSessionClosed
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn start_import_session_validation(
) -> std::result::Result<(
), ValidationErrors>
{

Ok((
))
}
/// StartImportSession - POST /image/importSession
#[tracing::instrument(skip_all)]
async fn start_import_session<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::image_import::ImageImport<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    start_import_session_validation(
    )
  ).await.unwrap();

  let Ok((
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



let result = api_impl.as_ref().start_import_session(
      
      &method,
      &host,
      &cookies,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::image_import::StartImportSessionResponse::Status201_ImportSessionStarted
                                                    (body)
                                                => {
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_images_validation(
  query_params: models::GetImagesQueryParams,
) -> std::result::Result<(
  models::GetImagesQueryParams,
), ValidationErrors>
{
  query_params.validate()?;

Ok((
  query_params,
))
}
/// GetImages - GET /images
#[tracing::instrument(skip_all)]
async fn get_images<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  QueryExtra(query_params): QueryExtra<models::GetImagesQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::image_list::ImageList<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_images_validation(
        query_params,
    )
  ).await.unwrap();

  let Ok((
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



let result = api_impl.as_ref().get_images(
      
      &method,
      &host,
      &cookies,
        &query_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::image_list::GetImagesResponse::Status200_Ok
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct AddImageTagsBodyValidator<'a> {
                #[validate(nested)]
          body: &'a Vec<models::NewImageTag>,
    }


#[tracing::instrument(skip_all)]
fn add_image_tags_validation(
  path_params: models::AddImageTagsPathParams,
        body: Vec<models::NewImageTag>,
) -> std::result::Result<(
  models::AddImageTagsPathParams,
        Vec<models::NewImageTag>,
), ValidationErrors>
{
  path_params.validate()?;
              let b = AddImageTagsBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
    body,
))
}
/// AddImageTags - POST /image/{id}/tags
#[tracing::instrument(skip_all)]
async fn add_image_tags<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::AddImageTagsPathParams>,
 State(api_impl): State<I>,
          Json(body): Json<Vec<models::NewImageTag>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::image_tag::ImageTag<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    add_image_tags_validation(
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



let result = api_impl.as_ref().add_image_tags(
      
      &method,
      &host,
      &cookies,
        &path_params,
              &body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::image_tag::AddImageTagsResponse::Status201_AllTagsOfImage
                                                    (body)
                                                => {
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::image_tag::AddImageTagsResponse::Status400_InvalidTagOrImageId
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_image_tags_validation(
  path_params: models::GetImageTagsPathParams,
) -> std::result::Result<(
  models::GetImageTagsPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// GetImageTags - GET /image/{id}/tags
#[tracing::instrument(skip_all)]
async fn get_image_tags<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
  Path(path_params): Path<models::GetImageTagsPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::image_tag::ImageTag<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_image_tags_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



let result = api_impl.as_ref().get_image_tags(
      
      &method,
      &host,
      &cookies,
        &path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::image_tag::GetImageTagsResponse::Status200_Ok
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::image_tag::GetImageTagsResponse::Status404_ImageNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_tags_validation(
) -> std::result::Result<(
), ValidationErrors>
{

Ok((
))
}
/// GetTags - GET /tags
#[tracing::instrument(skip_all)]
async fn get_tags<I, A, E>(
  method: Method,
  TypedHeader(host): TypedHeader<Host>,
  cookies: CookieJar,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::tag::Tag<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {




      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_tags_validation(
    )
  ).await.unwrap();

  let Ok((
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };



let result = api_impl.as_ref().get_tags(
      
      &method,
      &host,
      &cookies,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::tag::GetTagsResponse::Status200_Ok
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };


                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[allow(dead_code)]
#[inline]
fn response_with_status_code_only(code: StatusCode) -> Result<Response, StatusCode> {
   Response::builder()
          .status(code)
          .body(Body::empty())
          .map_err(|_| code)
}
