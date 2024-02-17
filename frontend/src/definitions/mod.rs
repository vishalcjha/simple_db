mod column;
pub mod table_definition;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColumnName(pub String);

impl From<&str> for ColumnName {
    fn from(value: &str) -> Self {
        ColumnName(String::from(value))
    }
}

pub(crate) trait NomParsable {
    fn nom_parse(input: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized;
}
