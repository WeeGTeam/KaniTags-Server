use kani_domain_api_incoming::image_management::ImageManagementService;
use kani_domain_api_incoming::login_service::LoginService;
use pantsu_domain::common::error::Error;
use pantsu_domain::reverse_image_search::ReverseImageSearchService;
use pantsu_openapi::apis::ErrorHandler;
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
    pub reverse_image_search_service: Arc<dyn ReverseImageSearchService + Send + Sync>,
    pub image_management_service: Arc<dyn ImageManagementService + Send + Sync>,
    pub login_service: Arc<dyn LoginService + Send + Sync>,
}

impl AppState {
    pub fn new<RS, IS, LS>(
        iqdb_service: Arc<RS>,
        image_management_service: Arc<IS>,
        login_service: Arc<LS>,
    ) -> Self
    where
        RS: ReverseImageSearchService + Send + Sync + 'static,
        IS: ImageManagementService + Send + Sync + 'static,
        LS: LoginService + Send + Sync + 'static,
    {
        Self {
            reverse_image_search_service: iqdb_service,
            image_management_service,
            login_service,
        }
    }
}
