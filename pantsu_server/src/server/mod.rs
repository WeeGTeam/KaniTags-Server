use utoipa_swagger_ui::SwaggerUi;

use crate::common::result::Result;
use crate::AppState;

mod multipart;
mod routes;

pub async fn launch_server(shared_state: AppState) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", shared_state.config.server_port)).await?;

    let (router, api) = routes::get_router(shared_state).split_for_parts();
    
    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));

    axum::serve(listener, router).await?;

    Ok(())
}
