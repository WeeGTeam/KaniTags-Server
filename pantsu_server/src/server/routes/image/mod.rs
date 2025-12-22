mod image;
mod import;
mod tags;

use crate::config::ServerConfig;
use crate::AppState;
use axum::extract::DefaultBodyLimit;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router(server_config: &ServerConfig) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(import::import)).route_layer(DefaultBodyLimit::max(server_config.request_body_limit.as_u64() as usize))
        .routes(routes!(image::dummy_get_image))    
        .routes(routes!(tags::dummy_get_tags))
}
