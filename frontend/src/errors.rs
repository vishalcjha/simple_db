use thiserror::Error;
pub type SError<T> = Result<T, DbError>;
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DbError {
    #[error("Unrecognized command {0}")]
    UnrecognizedCommand(String),
    #[error("Unrecognized statement command {0:?}")]
    StatementError(#[from] anyhow::Error),
    #[error("Unrecognized Column {0}")]
    UnrecognizedColumnType(String),
    #[error("Failed to do io operation")]
    IoError(#[from] std::io::Error),
}
