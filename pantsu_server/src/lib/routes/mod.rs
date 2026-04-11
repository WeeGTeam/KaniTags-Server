use crate::common::error::Error;
use crate::config::ServerConfig;
use crate::worker::fs::fs_service::FsService;
use crate::worker::iqdb::iqdb_service::IqdbService;
use pantsu_openapi::apis::ErrorHandler;
use std::sync::Arc;

pub mod image_download;
pub mod image_import;
pub mod image_list;
pub mod image_tag;

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
    pub iqdb_service: Arc<dyn IqdbService + Send + Sync>,
    pub fs_service: Arc<dyn FsService + Send + Sync>,
    pub config: ServerConfig,
}

impl AppState {
    pub fn new<I, F>(iqdb_service: Arc<I>, fs_service: Arc<F>, config: ServerConfig) -> Self
    where
        I: IqdbService + Send + Sync + 'static,
        F: FsService + Send + Sync + 'static,
    {
        Self {
            iqdb_service,
            fs_service,
            config,
        }
    }
}
