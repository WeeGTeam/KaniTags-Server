use diesel::result::{DatabaseErrorKind, Error as DieselError};
use kani_domain_api_outgoing::database::error::{DbError, ReadDbError, ReadWriteDbError, WriteDbError};

pub trait FromDieselError {
    fn from_diesel_error(error: anyhow::Error) -> Self;
}

impl FromDieselError for DbError {
    fn from_diesel_error(error: anyhow::Error) -> Self {
        DbError::Unknown(error)
    }
}

impl FromDieselError for WriteDbError {
    fn from_diesel_error(error: anyhow::Error) -> Self {
        if let Some(diesel_error) = error.root_cause().downcast_ref::<DieselError>() {
            match diesel_error {
                DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    return WriteDbError::UniqueViolation
                }
                _ => {}
            }
        }
        WriteDbError::Unknown(error)
    }
}

impl FromDieselError for ReadDbError {
    fn from_diesel_error(error: anyhow::Error) -> Self {
        if let Some(diesel_error) = error.root_cause().downcast_ref::<DieselError>() {
            match diesel_error {
                DieselError::NotFound => return ReadDbError::NotFound,
                _ => {}
            }
        }
        ReadDbError::Unknown(error)
    }
}

impl FromDieselError for ReadWriteDbError {
    fn from_diesel_error(error: anyhow::Error) -> Self {
        if let Some(diesel_error) = error.root_cause().downcast_ref::<DieselError>() {
            match diesel_error {
                DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    return ReadWriteDbError::UniqueViolation
                }
                DieselError::NotFound => return ReadWriteDbError::NotFound,
                _ => {}
            }
        }
        ReadWriteDbError::Unknown(error)
    }
}
