pub trait Execution {
    fn execute(&self);
}

pub fn execute(command: frontend::command::statement::StatementCommand) {
    match command {
        frontend::command::statement::StatementCommand::Select(statement) => statement.execute(),
        frontend::command::statement::StatementCommand::Insert(statement) => statement.execute(),
    }
}

impl Execution for frontend::insert::InsertStatement {
    fn execute(&self) {
        println!("Will execute insert -> {self:?}");
    }
}

impl Execution for frontend::select::SelectStatement {
    fn execute(&self) {
        println!("Will execute select -> {self:?}");
    }
}
