use crate::AppState;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

pub mod image;
pub mod images;
pub mod sauce;
pub mod tags;

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

pub fn get_router(app_state: AppState) -> OpenApiRouter {
    let config = &app_state.config;
    OpenApiRouter::with_openapi(ApiDoc::openapi())
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
