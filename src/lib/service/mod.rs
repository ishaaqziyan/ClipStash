use crate::{ClipError,DataError};

#[derive(Debug,thiserror::Error)]
pub enum ServiceError{
    #[error("clip error: {0}")]
    Clip(#[from]ClipError),
    #[error("database error: {0}")]
    Data(DataError),
    #[error("Not Found")]
    NotFound,
    #[error("Permissions not met yet:{0}")]
    PermissionError(String),

}


impl From<DataError> for ServiceError {
    fn from(err: DataError)-> Self {
        match err {
            DataError::Database(d) => match d {
                sqlx::Error::RowNotFound => Self::NotFound,
                other => Self::Data(DataError::Database(other))
            }
        }
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::NotFound,
            other => Self::Data(DataError::Database(other))
        }
    }
}