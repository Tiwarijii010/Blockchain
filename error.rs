use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum AppError {
    DatabaseError(String),
    TokenError(String),
    InvalidAmount,
    AmountExceedsLimit,
    ParseError(String),
    StorageError(String),
    IOError(String),
    NotFoundError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AppError {}

impl AppError {
    pub fn from_database_error(err: impl fmt::Debug) -> Self {
        AppError::DatabaseError(format!("{:?}", err))
    }

    pub fn from_token_error(err: impl fmt::Debug) -> Self {
        AppError::TokenError(format!("{:?}", err))
    }

    pub fn from_io_error(err: impl fmt::Debug) -> Self {
        AppError::IOError(format!("{:?}", err))
    }

    pub fn from_not_found_error(msg: &str) -> Self {
        AppError::NotFoundError(msg.to_string())
    }
}
