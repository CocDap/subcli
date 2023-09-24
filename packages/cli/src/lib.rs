use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    //TODO
}

pub fn execute(cmd: &Commands) -> Result<()> {
    //TODO

    Ok(())
}
