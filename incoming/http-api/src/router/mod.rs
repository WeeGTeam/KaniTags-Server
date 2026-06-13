use crate::error::HttpApiUnhandledError;
use axum::http::{Method, StatusCode};
use axum::response::Response;
use axum_extra::extract::CookieJar;
use headers::Host;
use kani_domain_api_incoming::collection_service::CollectionService;
use kani_domain_api_incoming::image_management::ImageManagementService;
use kani_domain_api_incoming::image_search_service::ImageSearchService;
use kani_domain_api_incoming::login_service::LoginService;
use kani_domain_api_incoming::similarity_service::SimilarityService;
use kani_domain_api_incoming::tag_service::TagService;
use kani_openapi::apis::ErrorHandler;
use std::sync::Arc;

pub mod collection;
pub mod image_download;
pub mod image_import;
pub mod image_tag;
pub mod image_list;
pub mod tag;

#[derive(Clone)]
pub struct OpenApiRouter(pub AppState);

impl AsRef<AppState> for OpenApiRouter {
    fn as_ref(&self) -> &AppState {
        &self.0
    }
}

#[async_trait::async_trait]
impl ErrorHandler<HttpApiUnhandledError> for AppState {
    async fn handle_error(&self, _method: &Method, _host: &Host, _cookies: &CookieJar, error: HttpApiUnhandledError) -> Result<Response, StatusCode> {
        match error {
            HttpApiUnhandledError::Unknown(_) => {
                tracing::error!("Unhandled error: {:?}", error);
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(axum::body::Body::empty())
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
            }
            HttpApiUnhandledError::GenericBadRequest(_) => {
                tracing::error!("Unhandled error: {:?}", error);
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(axum::body::Body::empty())
                    .map_err(|_| StatusCode::BAD_REQUEST)
            }
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub collection_service: Arc<dyn CollectionService + Send + Sync>,
    pub image_management_service: Arc<dyn ImageManagementService + Send + Sync>,
    pub image_search_service: Arc<dyn ImageSearchService + Send + Sync>,
    pub login_service: Arc<dyn LoginService + Send + Sync>,
    pub similarity_service: Arc<dyn SimilarityService + Send + Sync>,
    pub tag_service: Arc<dyn TagService + Send + Sync>,
}

impl AppState {
    pub fn new<CS, IS, ISS, LS, SS, TS>(
        collection_service: Arc<CS>,
        image_management_service: Arc<IS>,
        image_search_service: Arc<ISS>,
        login_service: Arc<LS>,
        similarity_service: Arc<SS>,
        tag_service: Arc<TS>,
    ) -> Self
    where
        CS: CollectionService + Send + Sync + 'static,
        IS: ImageManagementService + Send + Sync + 'static,
        ISS: ImageSearchService + Send + Sync + 'static,
        LS: LoginService + Send + Sync + 'static,
        SS: SimilarityService + Send + Sync + 'static,
        TS: TagService + Send + Sync + 'static,
    {
        Self {
            collection_service,
            image_management_service,
            image_search_service,
            login_service,
            similarity_service,
            tag_service,
        }
    }
}
