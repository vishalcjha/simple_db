use clap::Parser;

#[derive(Parser)]
pub(crate) struct Cli {
    pub db_path: std::path::PathBuf,
}
