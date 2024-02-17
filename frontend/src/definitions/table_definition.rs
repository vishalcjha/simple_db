use std::str::FromStr;

use anyhow::anyhow;
use nom::{
    bytes::complete::tag,
    character::{
        complete::{alpha1, space1},
        streaming::space0,
    },
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

use crate::errors::DbError;

use super::{column::Column, NomParsable};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TableName(pub String);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TableDefination {
    name: TableName,
    columns: Vec<Column>,
}

fn parse_columns(columns: &str) -> IResult<&str, Vec<Column>> {
    let (left, columns) = delimited(
        tag("("),
        separated_list1(tag(","), Column::nom_parse),
        tag(")"),
    )(columns)?;
    Ok((left, columns))
}

impl NomParsable for TableDefination {
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        let (left, (_, _, _, _, table_name, _, columns)) = tuple((
            tag("create"),
            space1,
            tag("table"),
            space1,
            alpha1,
            space0,
            parse_columns,
        ))(input)?;

        Ok((
            left,
            TableDefination {
                name: TableName(String::from(table_name)),
                columns,
            },
        ))
    }
}

impl FromStr for TableDefination {
    type Err = DbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TableDefination::nom_parse(s)
            .map_err(|err| anyhow!(format!("{err}")))?
            .1)
    }
}

#[cfg(test)]
mod test {

    use crate::definitions::{
        column::{Column, ColumnType},
        table_definition::TableName,
        NomParsable,
    };

    use super::TableDefination;
    #[test]
    fn test_successful() -> Result<(), String> {
        let create_command = "create table    test ( col1 int, col2 text, col3 int);";

        let result =
            TableDefination::nom_parse(create_command).map_err(|err| format!("{:?}", err))?;

        assert_eq!(
            TableDefination {
                name: TableName(String::from("test")),
                columns: vec![
                    Column::new("col1", ColumnType::Int),
                    Column::new("col2", ColumnType::Text),
                    Column::new("col3", ColumnType::Int)
                ]
            },
            result.1
        );

        Ok(())
    }
}
