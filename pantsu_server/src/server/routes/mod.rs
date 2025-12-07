use crate::AppState;
use axum::Router;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;

pub mod image;
pub mod images;
pub mod sauce;
pub mod tags;

pub fn get_router(app_state: AppState) -> Router {
    let config = &app_state.config;
    Router::new()
        .nest("/image", image::router(config))
        .nest("/images", images::router())
        .nest("/sauce", sauce::router())
        .nest("/tags", tags::router())
        .with_state(app_state.clone())
        .layer(TraceLayer::new_for_http()
            .make_span_with(crate::log::request_id::request_id_tracing_span)
        )
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid::default()))
        .layer(PropagateRequestIdLayer::x_request_id())
}
