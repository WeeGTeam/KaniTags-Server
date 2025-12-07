use crate::common::result::Result;
use axum::Json;

pub async fn get_images() -> Result<Json<String>> {
    Ok(Json(String::new()))
}
