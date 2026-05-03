use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

#[derive(Error, Debug)]
pub enum Error {
    // placeholder for anyhow error
    #[error("Internal server error: '{0}'")]
    Unknown(String),

    #[error("Internal server error: replace this error")]
    TodoError(),

    // filesystem
    #[error("Image exists on disk, but should not according to database: {0}")]
    UnexpectedImageExists(String),

    // channel
    #[error("channel communication error: {0}")]
    WorkerCommunicationError(String),
}


impl <T> From<mpsc::error::SendError<T>> for Error {
    fn from(value: mpsc::error::SendError<T>) -> Self {
        Self::WorkerCommunicationError(format!("send failed: {}", value))
    }
}

impl From<oneshot::error::RecvError> for Error {
    fn from(value: oneshot::error::RecvError) -> Self {
        Self::WorkerCommunicationError(format!("receive failed: {}", value))
    }
}
