use thiserror::Error;
use std::fmt;

pub mod blob;
pub mod commit;
pub mod tag;
pub mod tree;

/// Re-exports
pub use blob::BlobObject;
pub use commit::CommitObject;
pub use tag::TagObject;
pub use tree::TreeObject;

#[derive(Debug)]
pub enum ObjectType {
    Tree,
    Blob,
    Commit,
    Tag,
}

#[derive(Error, Debug)]
pub struct InvalidObjectError(String);

impl From<InvalidObjectError> for String {
    fn from(error: InvalidObjectError) -> String {
        return error.0
    }
}

impl fmt::Display for InvalidObjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl TryFrom<&str> for ObjectType {
    type Error = InvalidObjectError;

    fn try_from(value: &str) -> Result<ObjectType, Self::Error> {
        match value {
            "tree" => Ok(ObjectType::Tree),
            "blob" => Ok(ObjectType::Blob),
            "commit" => Ok(ObjectType::Commit),
            "tag" => Ok(ObjectType::Tag),
            _ => Err(InvalidObjectError(format!("Unknown object type {}", value)))
        }
    }
}
pub trait GitObject {
    const TYPE: &'static str;
}

#[derive(Debug, Clone)]
pub enum Object {
    Tree(tree::TreeObject),
    Blob(blob::BlobObject),
    Commit(commit::CommitObject),
    Tag(tag::TagObject),
}

impl From<Object> for Vec<u8> {
    fn from(object: Object) -> Vec<u8> {
        match object {
            Object::Tree(t) => t.into(),
            Object::Blob(b) => b.into(),
            Object::Commit(c) => c.into(),
            Object::Tag(t) => t.into(),
        }
    }
}

impl Object {
    pub fn get_type(&self) -> &'static str {
        match self {
            Object::Tree(_) => TreeObject::TYPE,
            Object::Blob(_) => BlobObject::TYPE,
            Object::Commit(_) => CommitObject::TYPE,
            Object::Tag(_) => TagObject::TYPE,
        }
    }
}
