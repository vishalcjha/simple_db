use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{alphanumeric1, space0, space1},
    sequence::tuple,
};

use crate::errors::DbError;

use super::NomParsable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Column(String, ColumnType);

impl NomParsable for Column {
    fn nom_parse(input: &str) -> nom::IResult<&str, Self> {
        let (left, (_, name, _, col_type)) =
            tuple((space0, alphanumeric1, space1, ColumnType::nom_parse))(input)?;

        Ok((left, Column(String::from(name), col_type)))
    }
}

impl Column {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, column_type: ColumnType) -> Column {
        Column(name.into(), column_type)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ColumnType {
    Int,
    Text,
}

impl FromStr for ColumnType {
    type Err = DbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ColumnType::*;
        let input = s.to_lowercase();
        match input.as_str() {
            "int" => Ok(Int),
            "text" => Ok(Text),
            _ => Err(DbError::UnrecognizedColumnType(format!("{s}"))),
        }
    }
}

impl NomParsable for ColumnType {
    fn nom_parse(input: &str) -> nom::IResult<&str, ColumnType> {
        let (left, type_name) = alt((tag_no_case("int"), tag_no_case("text")))(input)?;
        Ok((left, ColumnType::from_str(type_name).unwrap()))
    }
}
