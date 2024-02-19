use std::{path::PathBuf, str::FromStr};

use crate::errors::DbError;

#[derive(Debug, Clone)]
pub enum MetaCommand {
    Exit,
    Stats,
    DbPath(DbPath),
}

#[derive(Debug, Clone)]
pub struct DbPath(pub PathBuf);

impl FromStr for MetaCommand {
    type Err = DbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ".exit" => Ok(MetaCommand::Exit),
            _ => Err(DbError::UnrecognizedCommand(s.to_owned())),
        }
    }
}
