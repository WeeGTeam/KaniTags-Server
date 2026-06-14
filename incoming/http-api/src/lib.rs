use crate::auth_middleware::{AuthConfig, AuthState};
use crate::router::{AppState, OpenApiRouter};
use anyhow::Context;
use axum::extract::DefaultBodyLimit;
use axum::http::HeaderName;
use axum::middleware;
use kani_domain_api_incoming::image_management::ImageManagementService;
use kani_domain_api_incoming::image_search_service::ImageSearchService;
use kani_domain_api_incoming::login_service::LoginService;
use kani_domain_api_incoming::similarity_service::SimilarityService;
use kani_domain_api_incoming::tag_service::TagService;
use kani_openapi::server;
use std::str::FromStr;
use std::sync::Arc;
use tokio::signal;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;

pub mod auth_middleware;
pub mod converter;
mod error;
mod request_tracing;
pub mod router;

pub async fn launch_server<IS, ISS, LS, SS, TS>(
    image_management_service: Arc<IS>,
    image_search_service: Arc<ISS>,
    login_service: Arc<LS>,
    similarity_service: Arc<SS>,
    tag_service: Arc<TS>,
    request_body_limit: usize,
    server_port: u16,
    auth_user_header: String,
) -> Result<(), anyhow::Error>
where
    IS: ImageManagementService + Send + Sync + 'static,
    ISS: ImageSearchService + Send + Sync + 'static,
    LS: LoginService + Send + Sync + 'static,
    SS: SimilarityService + Send + Sync + 'static,
    TS: TagService + Send + Sync + 'static,
{
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", server_port))
        .await
        .with_context(|| format!("Failed to bind to port {}", server_port))?;

    let auth_layer = middleware::from_fn_with_state(
        AuthState {
            login_service: login_service.clone(),
            auth_config: AuthConfig {
                user_header: HeaderName::from_str(&auth_user_header).context("Invalid auth user header name configured")?,
            },
        },
        auth_middleware::auth_middleware
    );
    let shared_state = AppState::new(
        image_management_service,
        image_search_service,
        login_service,
        similarity_service,
        tag_service,
    );

    let body_limit = DefaultBodyLimit::max(request_body_limit);
    let router = server::new(OpenApiRouter(shared_state))
        .route_layer(auth_layer)
        .layer(TraceLayer::new_for_http().make_span_with(request_tracing::request_tracing_span))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid::default()))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(body_limit);

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Failed to launch service")?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
