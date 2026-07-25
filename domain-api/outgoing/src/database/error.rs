use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum ReadDbError {
    #[error("entity not found")]
    NotFound,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum WriteDbError {
    #[error("unique constraint violation")]
    UniqueViolation,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum ReadWriteDbError {
    #[error("entity not found")]
    NotFound,
    #[error("unique constraint violation")]
    UniqueViolation,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
