use crate::auth_middleware::{AuthConfig, AuthState};
use crate::router::{AppState, OpenApiRouter};
use axum::extract::DefaultBodyLimit;
use axum::http::HeaderName;
use axum::middleware;
use kani_domain_api_incoming::image_management::ImageManagementService;
use kani_domain_api_incoming::login_service::LoginService;
use pantsu_domain::common::error::Error;
use pantsu_domain::common::result::Result;
use pantsu_domain::reverse_image_search::ReverseImageSearchService;
use pantsu_openapi::server;
use std::str::FromStr;
use std::sync::Arc;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;

pub mod auth_middleware;
mod request_id;
pub mod router;

pub async fn launch_server<RS, IS, LS>(
    reverse_image_search_service: Arc<RS>,
    image_management_service: Arc<IS>,
    login_service: Arc<LS>,
    request_body_limit: usize,
    server_port: u16,
    auth_user_header: String,
) -> Result<()>
where
    RS: ReverseImageSearchService + Send + Sync + 'static,
    IS: ImageManagementService + Send + Sync + 'static,
    LS: LoginService + Send + Sync + 'static,
{
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", server_port))
        .await
        .map_err(|_| Error::TodoError())?;

    let auth_layer = middleware::from_fn_with_state(
        AuthState {
            login_service: login_service.clone(),
            auth_config: AuthConfig {
                user_header: HeaderName::from_str(&auth_user_header).map_err(|_| Error::TodoError())?,
            },
        },
        auth_middleware::auth_middleware
    );
    let shared_state = AppState::new(
        reverse_image_search_service,
        image_management_service,
        login_service,
    );

    let body_limit = DefaultBodyLimit::max(request_body_limit);
    let router = server::new(OpenApiRouter(shared_state))
        .route_layer(auth_layer)
        .layer(TraceLayer::new_for_http().make_span_with(request_id::request_id_tracing_span))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid::default()))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(body_limit);

    axum::serve(listener, router)
        .await
        .map_err(|_| Error::TodoError())?;

    Ok(())
}
