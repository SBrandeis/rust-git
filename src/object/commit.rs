use super::*;

#[derive(Debug, Clone)]
pub struct CommitObject {
    _raw: Box<[u8]>,
}

impl GitObject for CommitObject {
    const TYPE: &'static str = "commit";
}

impl Serializable for CommitObject {
    fn serialize(&self) -> &[u8] {
        unimplemented!()
    }

    fn deserialize(data: &[u8]) -> Self {
        return Self {
            _raw: Box::from(data),
        };
    }
}

impl From<CommitObject> for Object {
    fn from(obj: CommitObject) -> Object {
        Object::Commit(obj)
    }
}
