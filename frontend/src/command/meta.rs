use std::str::FromStr;

use crate::errors::DbError;

#[derive(Debug, Clone)]
pub enum MetaCommand {
    Exit,
}

impl FromStr for MetaCommand {
    type Err = DbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ".exit" => Ok(MetaCommand::Exit),
            _ => Err(DbError::UnrecognizedCommand(s.to_owned())),
        }
    }
}
