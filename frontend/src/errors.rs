use thiserror::Error;
pub type SError<T> = Result<T, DbError>;
#[derive(Debug, Error)]
pub enum DbError {
    #[error("Unrecognized command {0}")]
    UnrecognizedCommand(String),
    #[error("Failed to do io operation")]
    IoError(#[from] std::io::Error),
}
