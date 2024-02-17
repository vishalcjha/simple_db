use std::str::FromStr;

use crate::errors::DbError;

#[derive(Debug, Clone)]
pub struct InsertStatement(String);

impl FromStr for InsertStatement {
    type Err = DbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InsertStatement(s.to_owned()))
    }
}
