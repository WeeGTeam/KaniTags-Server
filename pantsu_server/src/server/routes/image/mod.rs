mod image;
mod import;
mod tags;

use crate::config::ServerConfig;
use crate::AppState;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::Router;

pub fn router(server_config: &ServerConfig) -> Router<AppState> {
    Router::new()
        .route("/", get(image::dummy_get_image))
        .route("/import", post(import::import)
            .layer(DefaultBodyLimit::max(server_config.request_body_limit.as_u64() as usize))
        )
        .route("/tags", get(tags::dummy_get_tags))
}
