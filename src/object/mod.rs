pub mod blob;
pub mod commit;
pub mod tag;
pub mod tree;

/// Re-exports
pub use blob::BlobObject;
pub use commit::CommitObject;
pub use tag::TagObject;
pub use tree::TreeObject;

pub trait Serializable {
    fn serialize(&self) -> &[u8];
    fn deserialize(data: &[u8]) -> Self;
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

impl Serializable for Object {
    fn serialize(&self) -> &[u8] {
        match self {
            Object::Tree(t) => t.serialize(),
            Object::Blob(b) => b.serialize(),
            Object::Commit(c) => c.serialize(),
            Object::Tag(t) => t.serialize(),
        }
    }

    fn deserialize(data: &[u8]) -> Self {
        unimplemented!();
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
