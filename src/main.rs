use std::{fs::read_to_string, str::FromStr};

use clap::Parser;
use colored::Colorize;
use frontend::{command::Command, errors::SError};
use rustyline::{error::ReadlineError, DefaultEditor};
use tracing::instrument;
mod cli;
mod help;
fn main() -> SError<()> {
    println!(
        "{} {} {}",
        "type".bold().green(),
        "help".yellow().bold().italic(),
        " to get started".bold().green()
    );
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("failed to install tracing");

    let cli = cli::Cli::parse();
    let db_exists = cli.db_path.exists();
    if !db_exists {
        load_sample_data();
    }

    backend::vm::initialize_db(frontend::DbPath(cli.db_path)).expect("Failed to initialize db");

    // add code to save loaded data.
    if !db_exists {
        backend::vm::persist_to_db().expect("Failed to persist to disk");
    }

    let mut rl = DefaultEditor::new().expect("Failed to open readline editor");
    let _ = rl.load_history("history.txt");
    loop {
        tracing::info!("reading lines");
        let readline = rl.readline("db > ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                execute_command(&line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    let _ = rl.save_history("history.txt");
    Ok(())
}

#[instrument(name = "test")]
fn execute_command(prompt: &str) {
    if prompt == "help" {
        help::print_help();
        return;
    }
    let command_result = Command::from_str(&prompt);
    let Ok(command) = command_result else {
        println!("{:?}", command_result.err().unwrap());
        return;
    };

    match command {
        Command::Meta(meta_command) => match meta_command {
            _ => {
                backend::vm::persist_to_db().expect("failed to save to db");
                std::process::exit(0);
            }
        },
        Command::Statement(statement) => {
            if let Err(err) = backend::vm::execute(statement) {
                println!("Failed with {err:?}");
            }
        }
    }
}

#[instrument]
fn load_sample_data() {
    tracing::info!("");
    for command in read_to_string("scripts/initial_load_student_table.sql")
        .unwrap()
        .lines()
    {
        let Ok(command) = Command::from_str(command) else {
            panic!("Failed to read pre load command");
        };

        match command {
            Command::Meta(_) => panic!("only statement command are allowed in pre-load"),
            Command::Statement(statement) => {
                backend::vm::execute(statement).expect("Failed to execute statement")
            }
        };
    }
}
