use crate::router::{AppState, OpenApiRouter};
use axum::extract::DefaultBodyLimit;
use pantsu_domain::common::error::Error;
use pantsu_domain::common::result::Result;
use pantsu_domain::library::LibraryService;
use pantsu_domain::reverse_image_search::ReverseImageSearchService;
use pantsu_openapi::server;
use std::sync::Arc;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;

mod request_id;
pub mod router;

pub async fn launch_server<RS, LS>(
    reverse_image_search_service: Arc<RS>,
    library_service: Arc<LS>,
    request_body_limit: usize,
    server_port: u16,
) -> Result<()>
where
    RS: ReverseImageSearchService + Send + Sync + 'static,
    LS: LibraryService + Send + Sync + 'static,
{
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", server_port))
        .await
        .map_err(|_| Error::TodoError())?;

    let shared_state = AppState::new(reverse_image_search_service, library_service);
    let body_limit = DefaultBodyLimit::max(request_body_limit);
    let router = server::new(OpenApiRouter(shared_state))
        .layer(TraceLayer::new_for_http().make_span_with(request_id::request_id_tracing_span))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid::default()))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(body_limit);

    axum::serve(listener, router).await
        .map_err(|_| Error::TodoError())?;

    Ok(())
}
