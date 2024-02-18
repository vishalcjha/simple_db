use std::{
    fs::read_to_string,
    io::{self, Write},
    str::FromStr,
};

use frontend::{command::Command, errors::SError, prompt::accept_prompt};
fn main() -> SError<()> {
    load_sample_data();
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
            Command::Statement(statement) => {
                if let Err(err) = backend::vm::execute(statement) {
                    println!("Failed with {err:?}");
                }
            }
        }
    }
}

fn load_sample_data() {
    for command in read_to_string("scripts/initial_load_student_table.sql")
        .unwrap()
        .lines()
    {
        let Ok(command) = Command::from_str(command) else {
            panic!("Failed to read pre load command");
        };

        match command {
            Command::Meta(_) => panic!("only statment command are allowed in pre-load"),
            Command::Statement(statement) => {
                backend::vm::execute(statement).expect("Failed to execute statement")
            }
        };
    }
}
