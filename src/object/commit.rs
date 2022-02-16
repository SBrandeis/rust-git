use super::*;

#[derive(Debug, Clone)]
pub struct CommitObject {
    _raw: Vec<u8>,
}

impl GitObject for CommitObject {
    const TYPE: &'static str = "commit";
}

impl From<CommitObject> for Vec<u8> {
    fn from(object: CommitObject) -> Vec<u8> {
        return object._raw;
    }
}

impl From<CommitObject> for Object {
    fn from(obj: CommitObject) -> Object {
        Object::Commit(obj)
    }
}

impl TryFrom<&[u8]> for CommitObject {
    type Error = InvalidObjectError;
    
    fn try_from(data: &[u8]) -> Result<CommitObject, Self::Error> {
        return Ok(CommitObject{_raw: Vec::from(data)})
    }
}

impl TryFrom<Vec<u8>> for CommitObject {
    type Error = InvalidObjectError;

    fn try_from(data: Vec<u8>) -> Result<CommitObject, Self::Error> {
        return Ok(CommitObject{_raw: data})
    }
}
