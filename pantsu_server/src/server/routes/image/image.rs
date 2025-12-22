use axum::Json;

use crate::common::result::Result;

#[utoipa::path(
    get,
    path = "/",
)]
pub async fn dummy_get_image() -> Result<Json<String>> {
    Ok(Json(String::new()))
}
