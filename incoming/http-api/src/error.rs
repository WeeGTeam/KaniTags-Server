use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpApiUnhandledError {
    #[error("Http api unhandled internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
    #[error("Http api client error: '{0}'")]
    GenericBadRequest(#[source] anyhow::Error),
    #[error("Http api not authorized error: '{0}'")]
    GenericForbidden(#[source] anyhow::Error),
    #[error("Http api not found error: '{0}'")]
    GenericNotFound(#[source] anyhow::Error),
}
