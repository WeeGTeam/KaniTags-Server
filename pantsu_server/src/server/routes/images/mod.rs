mod images;

use crate::AppState;
use axum::routing::get;
use utoipa_axum::router::OpenApiRouter;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .route("/", get(images::get_images))
}
