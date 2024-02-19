use std::{fmt::Debug, path::PathBuf};

use frontend::command::statement::StatementCommand;
use prettytable::{Cell, Row};

use crate::{errors::BEResult, DATABASE};
pub trait Execution {
    type Output;
    fn execute(self) -> BEResult<Self::Output>;
}

#[derive(Debug, Clone)]
pub enum ExecutionResult {
    SelectResult(()),
    InsertResult(()),
    CreateResult(()),
}

pub fn execute(command: StatementCommand) -> BEResult<ExecutionResult> {
    match command {
        StatementCommand::Select(statement) => statement
            .execute()
            .map(|_it| ExecutionResult::SelectResult(())),
        StatementCommand::Insert(statement) => statement
            .execute()
            .map(|_it| ExecutionResult::InsertResult(())),
        StatementCommand::Create(statement) => statement
            .execute()
            .map(|_it| ExecutionResult::CreateResult(())),
    }
}

pub fn initialize_db(db_path: frontend::DbPath) -> BEResult<()> {
    DATABASE.init_db_with_file(db_path.0)
}

impl Execution for frontend::InsertStatement {
    type Output = ();
    fn execute(self) -> BEResult<()> {
        DATABASE.insert_record(self)
    }
}

impl Execution for frontend::DbPath {
    type Output = ();

    fn execute(self) -> BEResult<Self::Output> {
        todo!()
    }
}

impl Execution for frontend::SelectStatement {
    type Output = ();
    fn execute(self) -> BEResult<()> {
        use prettytable::Table;
        let mut table = Table::new();
        table.add_row(Row::new(
            self.1.iter().map(|it| Cell::new(&it.0)).collect::<Vec<_>>(),
        ));
        let rows = DATABASE.select_records(self)?;
        for row in rows {
            table.add_row(Row::new(
                row.into_iter().map(|it| Cell::new(&it.value())).collect(),
            ));
        }

        table.printstd();
        Ok(())
    }
}

impl Execution for frontend::TableDefinition {
    type Output = ();
    fn execute(self) -> BEResult<()> {
        println!("Table def");
        println!("{self}");
        DATABASE.add_table_definitions(self)?;
        Ok(())
    }
}
