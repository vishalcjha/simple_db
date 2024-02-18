use frontend::definitions::table_definition::TableName;
use thiserror::Error;

pub(crate) type BEResult<T> = Result<T, BEErrors>;
#[derive(Debug, Error)]
pub enum BEErrors {
    #[error("table {0:?} already present in database")]
    DuplicateDefinition(TableName),
    #[error("Insert to table failed")]
    InsertFailed,
    #[error("Column {0} not present in table")]
    MissingColumn(String),
    #[error("{2} for Column {0} can not be conveted to {1}")]
    MismatchedDataType(String, &'static str, String),
    #[error("Missing table {0}")]
    MissingTable(String),
    #[error("Error in implementation {0}")]
    InternalError(String),
}
