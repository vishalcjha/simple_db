use frontend::command::statement::StatementCommand;
pub trait Execution {
    fn execute(&self);
}

pub fn execute(command: StatementCommand) {
    match command {
        StatementCommand::Select(statement) => statement.execute(),
        StatementCommand::Insert(statement) => statement.execute(),
        StatementCommand::Create(statement) => statement.execute(),
    }
}

impl Execution for frontend::InsertStatement {
    fn execute(&self) {
        println!("Will execute insert -> {self:?}");
    }
}

impl Execution for frontend::SelectStatement {
    fn execute(&self) {
        println!("Will execute select -> {self:?}");
    }
}

impl Execution for frontend::TableDefination {
    fn execute(&self) {
        println!("Will execute create -> {self:?}");
    }
}
