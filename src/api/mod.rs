//  Api - mod
use thiserror::Error;

pub mod task;

// error type made with this error
#[derive(Error, Debug)]
pub enum AppError {
    // created for managing postgres error
    #[error("{0}")]
    PostgresError(#[from] tokio_postgres::Error),
}
