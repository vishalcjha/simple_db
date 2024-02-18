use std::str::FromStr;

use frontend::TableDefinition;
use rstest::*;

#[fixture]
pub fn student_table_fixture() -> TableDefinition {
    TableDefinition::from_str("create table student(name text, age int);").unwrap()
}
