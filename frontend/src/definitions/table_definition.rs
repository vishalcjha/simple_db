use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

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

use prettytable::{row, Table};

use crate::errors::DbError;

use super::{column::Column, NomParsable};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TableName(pub String);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TableDefinition {
    pub name: TableName,
    pub columns: Vec<Column>,
}

impl Display for TableDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["column", "type"]);
        for col in self.columns.iter() {
            table.add_row(row![&col.0, &col.1]);
        }
        std::fmt::Display::fmt(&table, f)
    }
}

fn parse_columns(columns: &str) -> IResult<&str, Vec<Column>> {
    let (left, columns) = delimited(
        tag("("),
        separated_list1(tag(","), Column::nom_parse),
        tag(")"),
    )(columns)?;
    Ok((left, columns))
}

impl NomParsable for TableDefinition {
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
            TableDefinition {
                name: TableName(String::from(table_name)),
                columns,
            },
        ))
    }
}

impl FromStr for TableDefinition {
    type Err = DbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TableDefinition::nom_parse(s)
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

    use super::TableDefinition;
    #[test]
    fn test_successful() -> Result<(), String> {
        let create_command = "create table    test ( col1 int, col2 text, col3 int);";

        let result =
            TableDefinition::nom_parse(create_command).map_err(|err| format!("{:?}", err))?;

        assert_eq!(
            TableDefinition {
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
