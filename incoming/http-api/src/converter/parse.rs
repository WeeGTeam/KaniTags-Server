use crate::error::HttpApiUnhandledError;
use std::num::ParseIntError;

pub fn parse_id<T>(value: &str, constructor: impl FnOnce(i64) -> T) -> Result<T, HttpApiUnhandledError> {
    value.parse::<i64>().map(constructor)
        .map_err(|e: ParseIntError| HttpApiUnhandledError::GenericBadRequest(e.into()))
}
