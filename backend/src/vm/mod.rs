use std::fmt::Debug;

use frontend::command::statement::StatementCommand;
use prettytable::{Cell, Row};
use tracing::instrument;

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

#[instrument]
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

pub fn persist_to_db() -> BEResult<()> {
    DATABASE.flush_db()
}

impl Execution for frontend::InsertStatement {
    type Output = ();
    #[instrument]
    fn execute(self) -> BEResult<()> {
        tracing::info!("");
        DATABASE.insert_record(self)
    }
}

impl Execution for frontend::SelectStatement {
    type Output = ();
    #[instrument]
    fn execute(self) -> BEResult<()> {
        tracing::info!("");
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
    #[instrument]
    fn execute(self) -> BEResult<()> {
        tracing::info!("");
        println!("Table def");
        println!("{self}");
        DATABASE.add_table_definitions(self)?;
        Ok(())
    }
}
