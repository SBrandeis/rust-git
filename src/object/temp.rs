use super::*;

pub trait Serializable {
    const obj_type: &'static str;
    fn serialize(&self) -> &[u8];

    fn deserialize(data: &[u8]) -> Self;
}

#[derive(Debug, Clone)]
pub enum GitObject {
    Tree(TreeObject),
    Blob(BlobObject),
    Commit(CommitObject),
    Tag(TagObject),
}

#[derive(Debug, Clone)]
pub struct TreeObject {
    content: Vec<u8>,
}

impl From<TreeObject> for GitObject {
    fn from(obj: TreeObject) -> GitObject {
        GitObject::Tree(obj)
    }
}

impl Serializable for TreeObject {
    const obj_type: &'static str = "tree";

    fn serialize(&self) -> &[u8] {
        unimplemented!()
    }

    fn deserialize(data: &[u8]) -> Self {
        return Self {
            content: Vec::from(data),
        };
    }
}

#[derive(Debug, Clone)]
pub struct BlobObject {
    content: Vec<u8>,
}

impl From<BlobObject> for GitObject {
    fn from(obj: BlobObject) -> GitObject {
        GitObject::Blob(obj)
    }
}

impl Serializable for BlobObject {
    const obj_type: &'static str = "blob";
    fn serialize(&self) -> &[u8] {
        unimplemented!()
    }

    fn deserialize(data: &[u8]) -> Self {
        return Self {
            content: Vec::from(data),
        };
    }
}

#[derive(Debug, Clone)]
pub struct CommitObject {
    content: Vec<u8>,
}

impl From<CommitObject> for GitObject {
    fn from(obj: CommitObject) -> GitObject {
        GitObject::Commit(obj)
    }
}

impl Serializable for CommitObject {
    const obj_type: &'static str = "commit";
    fn serialize(&self) -> &[u8] {
        unimplemented!()
    }

    fn deserialize(data: &[u8]) -> Self {
        return Self {
            content: Vec::from(data),
        };
    }
}

#[derive(Debug, Clone)]
pub struct TagObject {
    content: Vec<u8>,
}

impl From<TagObject> for GitObject {
    fn from(obj: TagObject) -> GitObject {
        GitObject::Tag(obj)
    }
}

impl Serializable for TagObject {
    const obj_type: &'static str = "tag";
    fn serialize(&self) -> &[u8] {
        unimplemented!()
    }

    fn deserialize(data: &[u8]) -> Self {
        return Self {
            content: Vec::from(data),
        };
    }
}
