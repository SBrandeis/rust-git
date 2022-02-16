extern crate clap;

use clap::Parser;

pub mod commands;
pub mod object;
pub mod repository;
use std::error::Error;
use std::process;
use commands::Command;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: commands::Commands
}


fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let rc = match args.command {
        commands::Commands::CatFile(command) => command.exec()?,
    };
    process::exit(rc as i32);
}
