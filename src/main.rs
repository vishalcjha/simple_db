use std::{
    io::{self, Write},
    str::FromStr,
};

use frontend::{command::Command, errors::SError, prompt::accept_prompt};
fn main() -> SError<()> {
    loop {
        print!("db > ");
        io::stdout().flush()?;
        let prompt = accept_prompt()?;
        let command_result = Command::from_str(&prompt);
        let Ok(command) = command_result else {
            println!("{:?}", command_result.err().unwrap());
            continue;
        };

        match command {
            Command::Meta(meta_command) => match meta_command {
                _ => std::process::exit(0),
            },
            Command::Statement(statement) => backend::vm::execute(statement),
        }
    }
}
