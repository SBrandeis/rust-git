use clap::{Subcommand};
use std::error;
pub mod cat_file;

use super::*;

pub trait Command {
    fn exec(&self) -> Result<u8, Box<dyn error::Error>>;
}

#[derive(Subcommand)]
pub enum Commands {
    CatFile(cat_file::CatFileCommand),
}
