use ini;
use std::io;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenError {
    #[error("provided path `{0}` does not exist")]
    NotFound(String),
    #[error(".git directory not found at path `{0}`")]
    GitDirNotFound(String),
    #[error("failed to parse git config")]
    ConfigParse(#[from] ini::Error),
    #[error("invalid git config ({0})")]
    InvalidConfig(String),
    #[error("unsupported repo format version `{0:?}`")]
    UnsupportedFormatVersion(Option<String>),
}

#[derive(Error, Debug)]
pub enum InitError {
    #[error("path `{0}` is not a directory")]
    NotADirectory(String),
    #[error("path `{0}` is not empty")]
    NotEmpty(String),
    #[error("io error")]
    IOError(#[from] io::Error),
}
