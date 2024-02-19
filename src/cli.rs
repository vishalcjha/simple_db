use clap::Parser;

#[derive(Parser)]
pub(crate) struct Cli {
    #[arg(short, long)]
    pub db_path: std::path::PathBuf,
}
