use std::str::FromStr;

use nom::{branch::alt, bytes::complete::tag_no_case, IResult};

use crate::{definitions::table_definition::TableDefinition, errors::DbError};

use self::{insert::InsertStatement, select::SelectStatement};

pub mod insert;
pub mod select;

#[derive(Debug, Clone)]
pub enum StatementCommand {
    Select(SelectStatement),
    Insert(InsertStatement),
    Create(TableDefinition),
}

impl FromStr for StatementCommand {
    type Err = DbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CommandType::*;
        let command_type = CommandType::from_str(s)?;
        match command_type {
            Select => Ok(StatementCommand::Select(SelectStatement::from_str(s)?)),
            Insert => Ok(StatementCommand::Insert(InsertStatement::from_str(s)?)),
            Create => Ok(StatementCommand::Create(TableDefinition::from_str(s)?)),
        }
    }
}

enum CommandType {
    Select,
    Insert,
    Create,
}

fn parse_select_command(command: &str) -> IResult<&str, CommandType> {
    let _ = tag_no_case("select")(command)?;
    Ok(("", CommandType::Select))
}

fn parse_insert_command(command: &str) -> IResult<&str, CommandType> {
    let _ = tag_no_case("insert")(command)?;
    Ok(("", CommandType::Insert))
}

fn parse_create_command(command: &str) -> IResult<&str, CommandType> {
    let _ = tag_no_case("create")(command)?;
    Ok(("", CommandType::Create))
}

fn parse_command_type(command: &str) -> IResult<&str, CommandType> {
    let command = alt((
        parse_select_command,
        parse_insert_command,
        parse_create_command,
    ))(command.trim())?
    .1;
    Ok(("", command))
}

impl FromStr for CommandType {
    type Err = DbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = parse_command_type(s)
            .map(|it| it.1)
            .map_err(|it| anyhow::anyhow!(format!("{:?}", it)))?;
        Ok(res)
    }
}
