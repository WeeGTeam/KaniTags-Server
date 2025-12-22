use axum::Json;

use crate::common::result::Result;

#[utoipa::path(
    get,
    path = "/tags",
)]
pub async fn dummy_get_tags() -> Result<Json<String>> {
    Ok(Json(String::new()))
}
