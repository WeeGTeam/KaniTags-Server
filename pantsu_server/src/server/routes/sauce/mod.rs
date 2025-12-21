use crate::AppState;
use utoipa_axum::router::OpenApiRouter;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
}
