use std::{
    io::{self, Write},
    process,
};

use frontend::{errors::SError, prompt::accept_prompt};
fn main() -> SError<()> {
    loop {
        print!("db > ");
        io::stdout().flush()?;
        let prompt = accept_prompt()?;
        match prompt.as_str() {
            ".exit" => {
                process::exit(0);
            }
            _ => {
                println!("Unrecognized command {prompt}");
            }
        }
    }
}
