use pantsu_domain::common::error::Error;
use pantsu_domain::library::LibraryService;
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
    pub library_service: Arc<dyn LibraryService + Send + Sync>,
}

impl AppState {
    pub fn new<RS, LS>(iqdb_service: Arc<RS>, fs_service: Arc<LS>) -> Self
    where
        RS: ReverseImageSearchService + Send + Sync + 'static,
        LS: LibraryService + Send + Sync + 'static,
    {
        Self {
            reverse_image_search_service: iqdb_service,
            library_service: fs_service,
        }
    }
}
