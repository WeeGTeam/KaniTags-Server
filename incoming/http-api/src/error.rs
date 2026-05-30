use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpApiUnhandledError {
    #[error("Http api unhandled internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
}

