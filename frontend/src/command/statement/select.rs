use std::str::FromStr;

use crate::errors::DbError;

#[derive(Debug, Clone)]
pub struct SelectStatement(String);

impl FromStr for SelectStatement {
    type Err = DbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SelectStatement(s.to_owned()))
    }
}
