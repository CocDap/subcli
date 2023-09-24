use clap::Parser;
use subcli::{execute, Cli};

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    execute(&cli.command)
}
