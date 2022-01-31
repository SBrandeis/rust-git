extern crate flate2;
extern crate ini;

use super::*;

use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use flate2::read::ZlibDecoder;
use ini::Ini;
use thiserror::Error;

use object::{BlobObject, CommitObject, GitObject, Serializable, TagObject, TreeObject};

const DEFAULT_DESCRIPTION: &str =
    "Unnamed repository; edit this file 'description' to name the repository.\n";
const DEFAULT_BRANCH: &str = "main";

#[derive(Debug, Clone)]
pub struct Repository {
    worktree: PathBuf,
    git_dir: PathBuf,
    config: config::RepoConfig,
}

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
            config: config::RepoConfig::try_from(config)?,
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

        config::RepoConfig::default()
            .as_ref()
            .write_to_file(git_dir_path.join("config"))?;
        fs::write(git_dir_path.join("description"), DEFAULT_DESCRIPTION)?;
        fs::write(
            git_dir_path.join("HEAD"),
            format!("ref: refs/heads/{}\n", DEFAULT_BRANCH),
        )?;

        Ok(Repository::open(path).unwrap())
    }

    pub fn find_parent(path: impl AsRef<Path>) -> Result<Option<Repository>, OpenError> {
        let path = path.as_ref();

        if path.join(".git").exists() {
            Ok(Some(Repository::open(path)?))
        } else if let Some(parent) = path.parent() {
            Repository::find_parent(parent)
        } else {
            Ok(None)
        }
    }

    pub fn object_read(
        &self,
        sha: &impl AsRef<str>,
    ) -> Result<Option<object::GitObject>, ReadObjectError> {
        let path2obj = self.git_dir.join(format!(
            "objects/{0}/{1}",
            &sha.as_ref()[0..2],
            &sha.as_ref()[2..],
        ));
        println!("{:?}", path2obj);
        let mut deflated_reader = match fs::File::open(path2obj) {
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => return Ok(None),
                _ => return Err(err)?,
            },
            Ok(file) => BufReader::new(ZlibDecoder::new(file)),
        };

        let mut obj_type: Vec<u8> = Vec::with_capacity(6);
        let mut obj_size: Vec<u8> = Vec::with_capacity(32);
        deflated_reader.read_until(0x20, &mut obj_type)?;
        deflated_reader.read_until(0x00, &mut obj_size)?;
        obj_type.pop();
        obj_size.pop();
        // ^^ TODO: make this cleaner - maybe with .split().next() or smth ?

        println!("obj_type: {:02x?}", obj_type);
        println!("obj_size: {:02x?}", obj_size);

        let obj_type = String::from_utf8(obj_type).or(Err(ReadObjectError::InvalidObject(
            format!("Could not parse object type for {0}", &sha.as_ref()),
        )))?;
        let obj_size: usize = String::from_utf8(obj_size)
            .or(Err(ReadObjectError::InvalidObject(format!(
                "Could not parse object type for ${0}",
                &sha.as_ref()
            ))))?
            .parse()
            .or(Err(ReadObjectError::InvalidObject(format!(
                "Could not parse object type for ${0}",
                &sha.as_ref()
            ))))?;

        let mut obj_content: Vec<u8> = Vec::with_capacity(obj_size);
        let actual_size = deflated_reader.read_to_end(&mut obj_content)?;
        if actual_size != obj_size {
            return Err(ReadObjectError::InvalidObject("Size mismatch".to_owned()));
        }

        let git_obj: GitObject = match obj_type.as_str() {
            "commit" => CommitObject::deserialize(&obj_content).into(),
            "blob" => BlobObject::deserialize(&obj_content).into(),
            "tree" => TreeObject::deserialize(&obj_content).into(),
            "tag" => TagObject::deserialize(&obj_content).into(),
            t => {
                return Err(ReadObjectError::InvalidObject(format!(
                    "Unknown object type: `${0}`",
                    t
                )))
            }
        };
        Ok(Some(git_obj))
    }
}

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

#[derive(Error, Debug)]
pub enum ReadObjectError {
    #[error("io error")]
    IOError(#[from] io::Error),
    #[error("Invalid object: `${0}`")]
    InvalidObject(String),
}

pub mod config {
    use super::*;

    #[derive(Clone)]
    pub struct RepoConfig(Ini);
    impl AsRef<Ini> for RepoConfig {
        fn as_ref(&self) -> &Ini {
            &self.0
        }
    }
    impl fmt::Debug for RepoConfig {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RepoConfig").finish()
        }
    }
    impl TryFrom<Ini> for RepoConfig {
        type Error = OpenError;
        fn try_from(cfg: Ini) -> Result<Self, Self::Error> {
            if let Some(fmt_version) = cfg
                .section(Some("core"))
                .ok_or(OpenError::InvalidConfig(String::from(
                    "missing core section",
                )))?
                .get("repositoryformatversion")
            {
                if fmt_version != "0" {
                    return Err(OpenError::UnsupportedFormatVersion(Some(String::from(
                        fmt_version,
                    ))));
                }
                Ok(RepoConfig(cfg))
            } else {
                return Err(OpenError::UnsupportedFormatVersion(None));
            }
        }
    }
    impl RepoConfig {
        pub fn default() -> Self {
            let mut cfg = Ini::new();
            cfg.with_section(Some("core"))
                .set("repositoryformatversion", "0")
                .set("filemode", "false")
                .set("bare", "false");
            RepoConfig(cfg)
        }
    }
}