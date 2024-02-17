use crate::errors::SError;
use lazy_static::lazy_static;
use rustyline::DefaultEditor;
use std::sync::Mutex;

struct PromptReader(Mutex<DefaultEditor>);
lazy_static! {
    static ref PROMPT_READER: PromptReader = {
        let mut rl = DefaultEditor::new().unwrap();
        let _ = rl.load_history("history.txt");

        PromptReader(Mutex::new(rl))
    };
}

pub fn accept_prompt() -> SError<String> {
    let mut rl = PROMPT_READER.0.lock().unwrap();
    let readline = rl.readline("db > ")?;

    rl.save_history("history.txt")?;

    Ok(readline)
}
