use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author = "Sanjay Krishnan",
    version = "0.1.0",
    about = "A cli tool for general directory information"
)]
pub struct Cli {
    ///Path to the directory, defaults to the current directory if omitted
    #[arg(default_value = ".")]
    pub path: PathBuf
}

