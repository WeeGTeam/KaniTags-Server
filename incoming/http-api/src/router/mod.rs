use kani_domain::common::error::Error;
use kani_domain_api_incoming::image_management::ImageManagementService;
use kani_domain_api_incoming::login_service::LoginService;
use kani_openapi::apis::ErrorHandler;
use std::sync::Arc;

pub mod image_download;
pub mod image_import;
pub mod image_tag;
pub mod image_list;

#[derive(Clone)]
pub struct OpenApiRouter(pub AppState);

impl AsRef<AppState> for OpenApiRouter {
    fn as_ref(&self) -> &AppState {
        &self.0
    }
}

impl ErrorHandler<Error> for AppState {}

#[derive(Clone)]
pub struct AppState {
    pub image_management_service: Arc<dyn ImageManagementService + Send + Sync>,
    pub login_service: Arc<dyn LoginService + Send + Sync>,
}

impl AppState {
    pub fn new<IS, LS>(
        image_management_service: Arc<IS>,
        login_service: Arc<LS>,
    ) -> Self
    where
        IS: ImageManagementService + Send + Sync + 'static,
        LS: LoginService + Send + Sync + 'static,
    {
        Self {
            image_management_service,
            login_service,
        }
    }
}
