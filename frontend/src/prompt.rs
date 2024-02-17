use std::io::{self, BufRead};

use crate::errors::SError;

pub fn accept_prompt() -> SError<String> {
    let mut prompt = String::new();
    io::stdin().lock().read_line(&mut prompt)?;

    // remove newline char
    prompt.pop();

    Ok(prompt)
}
