use std::fmt::Debug;

use frontend::command::statement::StatementCommand;

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

impl Execution for frontend::InsertStatement {
    type Output = ();
    fn execute(self) -> BEResult<()> {
        println!("Will execute insert -> {self:?}");
        Ok(())
    }
}

impl Execution for frontend::SelectStatement {
    type Output = ();
    fn execute(self) -> BEResult<()> {
        println!("Will execute select -> {self:?}");
        Ok(())
    }
}

impl Execution for frontend::TableDefinition {
    type Output = ();
    fn execute(self) -> BEResult<()> {
        DATABASE.add_table_definitions(self)?;
        Ok(())
    }
}
