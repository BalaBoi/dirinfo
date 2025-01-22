use clap::Parser;
use dirinfo::{cli::Cli, dir::Dir, setup_tracing_subscriber};

fn main() {
    setup_tracing_subscriber();
    let cli = Cli::parse();
    let dir = Dir::validate(cli.path).unwrap();
    let info = dir.info().unwrap();
    info.display(std::io::stdout().lock()).unwrap();
}
