use anyhow::Result;
use clap::Parser;
use dirinfo::{cli::Cli, Dir};

fn main() -> Result<()> {
    let cli = Cli::parse();
    // println!("Input path: {}", cli.path.display());
    let dir = Dir::validate(cli.path)?;
    let info = dir.info()?;
    info.display(std::io::stdout().lock())?;
    Ok(())
}
