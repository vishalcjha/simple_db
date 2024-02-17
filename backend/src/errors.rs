use frontend::definitions::table_definition::TableName;
use thiserror::Error;

pub(crate) type BEResult<T> = Result<T, BEErrors>;
#[derive(Debug, Error)]
pub enum BEErrors {
    #[error("table {0:?} already present in database")]
    DuplicateDefinition(TableName),
}
