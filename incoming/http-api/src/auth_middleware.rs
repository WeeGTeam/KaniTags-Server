use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderName, Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use kani_domain_api_incoming::login_service::{LoginService, UserLoadError};
use kani_domain_api_model::user::User;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState {
    pub login_service: Arc<dyn LoginService + Send + Sync>,
    pub auth_config: AuthConfig,
}

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub user_header: HeaderName,
}

tokio::task_local! {
    static CURRENT_USER: User
}

pub fn current_user() -> User {
    CURRENT_USER.get()
}

pub async fn auth_middleware(
    State(state): State<AuthState>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let header = &state.auth_config.user_header;
    let user_name = extract_user_name(&request, header);
    let user_name = match user_name {
        Some(name) => name,
        None => {
            tracing::error!("Missing user header: {header}");
            return Err((StatusCode::UNAUTHORIZED, "not authorized".to_owned()))
        }
    };

    let user_result = state.login_service.load_user_by_user_name(&user_name).await;
    let user = user_result.map_err(|e| match e {
        UserLoadError::Unknown(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        UserLoadError::UserMissingError(e) => (StatusCode::UNAUTHORIZED, e),
    })?;

    // Run the rest of the request inside the task-local scope.
    Ok(CURRENT_USER.scope(user, next.run(request)).await)
}

#[cfg(not(feature = "mock-user"))]
fn extract_user_name(request: &Request<Body>, header: &HeaderName) -> Option<String> {
    request
        .headers()
        .get(header)
        .and_then(|v| v.to_str().ok().map(|s| s.to_owned()))
}

#[cfg(feature = "mock-user")]
fn extract_user_name(_request: &Request<Body>, _header: &HeaderName) -> Option<String> {
    Some("anonymous".to_owned())
}
