use std::str::FromStr;

use anyhow::anyhow;
use nom::{
    bytes::{complete::tag_no_case, streaming::tag},
    character::complete::{alphanumeric1, char},
    multi::{many0, many1, separated_list0},
    sequence::{separated_pair, tuple},
    IResult,
};

use crate::{
    definitions::{table_definition::TableName, ColumnName},
    errors::DbError,
};

#[derive(Debug, Clone)]
pub struct SelectStatement(pub TableName, pub Vec<ColumnName>);

fn parse_table_name(statement: &str) -> IResult<&str, &str> {
    let table_name = tuple((
        tag_no_case("from"),
        many1(char(' ')),
        alphanumeric1,
        many0(char(' ')),
        char(';'),
    ))(statement)?
    .1
     .2;
    Ok(("", table_name))
}
fn parse_select_statement(statement: &str) -> IResult<&str, SelectStatement> {
    let left = tag_no_case("select")(statement)?.0.trim();
    let columns_table_name = separated_pair(
        separated_list0(
            tuple((many0(char(' ')), (tag(",")), many0(char(' ')))),
            alphanumeric1,
        ),
        many1(char(' ')),
        parse_table_name,
    )(left)?;

    let columns = columns_table_name
        .1
         .0
        .into_iter()
        .map(|it| ColumnName(String::from(it)))
        .collect::<Vec<_>>();
    let table_name = columns_table_name.1 .1.to_owned();

    Ok(("", SelectStatement(TableName(table_name), columns)))
}

impl FromStr for SelectStatement {
    type Err = DbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_select_statement(s)
            .map_err(|err| anyhow!(format!("{err}")))?
            .1)
    }
}

#[cfg(test)]
mod test {

    use crate::errors::SError;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_valid_select_with_single_column() -> SError<()> {
        let statement = "select name from student;";
        let parse_statement = SelectStatement::from_str(statement)?;

        assert_eq!(parse_statement.0 .0.as_str(), "student");
        let expected_columns: Vec<ColumnName> = vec!["name".into()];
        assert_eq!(expected_columns, parse_statement.1);

        Ok(())
    }

    #[rstest]
    #[case("select name,age from student;", vec!["name", "age"])]
    #[case("select name, age from student;", vec!["name", "age"])]
    #[case("select name ,age from student;", vec!["name", "age"])]
    #[case("select name ,  age from student;", vec!["name", "age"])]
    fn test_valid_select_with_multiple_columns(
        #[case] statement: &'static str,
        #[case] expected: Vec<&'static str>,
    ) -> SError<()> {
        let parse_statement = SelectStatement::from_str(statement)?;

        assert_eq!(parse_statement.0 .0.as_str(), "student");
        let expected_columns: Vec<ColumnName> = expected.into_iter().map(|it| it.into()).collect();
        assert_eq!(expected_columns, parse_statement.1);

        Ok(())
    }

    #[test]
    fn test_invalid_comma() -> SError<()> {
        let statement = "select name, from student;";
        let Err(err) = SelectStatement::from_str(statement) else {
            panic!("Error expected");
        };

        let is_statement_error = match err {
            DbError::StatementError(_) => true,
            _ => false,
        };
        assert_eq!(true, is_statement_error);

        Ok(())
    }
}
