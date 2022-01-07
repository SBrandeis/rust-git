extern crate ini;

use std::fmt::Display;
use std::path::{Path, PathBuf};

pub mod config;
pub mod error;

use config::RepoConfig;
use error::{InitError, OpenError};
use ini::Ini;
use std::fs;

#[derive(Debug, Clone)]
pub struct Repository {
    worktree: PathBuf,
    git_dir: PathBuf,
    config: RepoConfig,
}

const DEFAULT_DESCRIPTION: &str =
    "Unnamed repository; edit this file 'description' to name the repository.\n";
const DEFAULT_BRANCH: &str = "main";

impl Repository {
    /// Computes path under the repo's git dir
    ///
    /// # Examples
    /// ```
    /// repo.path(Path::new("objects/refs"))
    /// ```
    pub fn path(&self, relpath: impl AsRef<Path>) -> PathBuf {
        self.git_dir.join(relpath)
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, OpenError> {
        let path = path.as_ref();
        let git_dir_path = path.join(".git");
        let config_path = git_dir_path.join("config");

        if !path.exists() {
            return Err(OpenError::NotFound(path.to_string_lossy().to_string()));
        }
        if !git_dir_path.exists() {
            return Err(OpenError::GitDirNotFound(
                path.to_string_lossy().to_string(),
            ));
        }
        let config = Ini::load_from_file(git_dir_path.join("config"))?;

        Ok(Repository {
            worktree: AsRef::<Path>::as_ref(path).to_owned(),
            git_dir: git_dir_path,
            config: RepoConfig::try_from(config)?,
        })
    }

    pub fn init(path: impl AsRef<Path>) -> Result<Self, InitError> {
        let path = path.as_ref();

        if path.exists() {
            if path.is_file() {
                return Err(InitError::NotADirectory(path.to_string_lossy().to_string()));
            } else {
                let is_empty = path.read_dir()?.next().is_none();
                if !is_empty {
                    return Err(InitError::NotEmpty(path.to_string_lossy().to_string()));
                }
            }
        } else {
            fs::create_dir_all(path)?;
        }

        let git_dir_path = path.join(".git");

        fs::create_dir_all(git_dir_path.join("branches"))?;
        fs::create_dir_all(git_dir_path.join("objects"))?;
        fs::create_dir_all(git_dir_path.join("refs/tags"))?;
        fs::create_dir_all(git_dir_path.join("refs/heads"))?;

        RepoConfig::default()
            .as_ref()
            .write_to_file(git_dir_path.join("config"))?;
        fs::write(git_dir_path.join("description"), DEFAULT_DESCRIPTION)?;
        fs::write(
            git_dir_path.join("HEAD"),
            format!("ref: refs/heads/{}\n", DEFAULT_BRANCH),
        )?;

        Ok(Repository::open(path).unwrap())
    }

    fn find_parent(path: impl AsRef<Path>) -> Result<Option<Repository>, OpenError> {
        let path = path.as_ref();

        if path.join(".git").exists() {
            Ok(Some(Repository::open(path)?))
        } else if let Some(parent) = path.parent() {
            Repository::find_parent(parent)
        } else {
            Ok(None)
        }
    }
}
