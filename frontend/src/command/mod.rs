use std::str::FromStr;

use crate::errors::DbError;

use self::{meta::MetaCommand, statement::StatementCommand};

mod meta;
pub mod statement;

#[derive(Debug, Clone)]
pub enum Command {
    Meta(MetaCommand),
    Statement(StatementCommand),
}

impl FromStr for Command {
    type Err = DbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.starts_with(".") {
            return Ok(Command::Meta(MetaCommand::from_str(s)?));
        }
        Ok(Command::Statement(StatementCommand::from_str(s)?))
    }
}
