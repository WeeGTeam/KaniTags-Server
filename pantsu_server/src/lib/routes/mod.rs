use crate::config::ServerConfig;
use crate::worker::fs::fs_service::FsService;
use crate::worker::iqdb::iqdb_service::IqdbService;
use std::sync::Arc;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

pub mod image;
pub mod images;
pub mod sauce;
pub mod tags;

mod multipart;

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

pub fn get_router(config: &ServerConfig) -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/image", image::router(config))
        .nest("/images", images::router())
        .nest("/sauce", sauce::router())
        .nest("/tags", tags::router())
        .layer(TraceLayer::new_for_http()
            .make_span_with(crate::log::request_id::request_id_tracing_span)
        )
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid::default()))
        .layer(PropagateRequestIdLayer::x_request_id())
}

#[derive(Clone)]
pub struct AppState {
    pub iqdb_service: Arc<dyn IqdbService + Send + Sync>,
    pub fs_service: Arc<dyn FsService + Send + Sync>,
    pub config: ServerConfig,
}

impl AppState {
    pub fn new<I, F>(iqdb_service: Arc<I>, fs_service: Arc<F>, config: ServerConfig) -> Self
    where
    I: IqdbService + Send + Sync + 'static,
    F: FsService + Send + Sync + 'static
    {
        Self { iqdb_service, fs_service, config }
    }
}
