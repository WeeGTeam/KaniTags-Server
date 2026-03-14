use axum::body::Body;
use axum::extract::Path;
use axum::http::{header, HeaderValue};
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use tokio_util::io::ReaderStream;
use utoipa::ToSchema;

#[derive(ToSchema)]
#[schema(value_type = String, format = Binary)]
pub struct Binary(#[schema(inline)] String);

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Image id")
    ),
    responses(
        (
            status = OK,
            description = "Image file (png or jpg)",
            body = inline(Binary),
            content_type = "application/octet-stream"
        ),
        (status = INTERNAL_SERVER_ERROR, description = "Image file load failed")
    )
)]
pub async fn get_image(id: Path<ImageId>) -> impl IntoResponse {
    let file = tokio::fs::File::open("3b6368639f3e17fa-3803887ff7833837f03e43e43e21303b61fe.png")
        .await
        .unwrap();
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    let mut response = Response::new(body);
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("image/png"),
    );
    response
}

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/{id}/thumbnail",
    params(
        ("id" = String, Path, description = "Image id")
    ),
    responses(
        (
            status = OK,
            description = "Image thumbnail file (png or jpg)",
            body = inline(Binary),
            content_type = "application/octet-stream"
        ),
        (status = INTERNAL_SERVER_ERROR, description = "Image file load failed")
    )
)]
pub async fn get_image_thumbnail(id: Path<ImageId>) -> impl IntoResponse {
    let file = tokio::fs::File::open("3b6368639f3e17fa-3803887ff7833837f03e43e43e21303b61fe.png")
        .await
        .unwrap();
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    body
}

#[derive(Deserialize)]
#[serde(try_from = "String")]
pub struct ImageId {
    value: String,
}

impl TryFrom<String> for ImageId {
    type Error = String;

    fn try_from(image_id: String) -> Result<Self, Self::Error> {
        Ok(ImageId { value: image_id })
    }
}

pub enum ImageType {
    FULL,
    THUMBNAIL,
}
