use super::*;

#[derive(Debug, Clone)]
pub struct TreeObject {
    _raw: Vec<u8>,
}

impl GitObject for TreeObject {
    const TYPE: &'static str = "tree";
}

impl From<TreeObject> for Vec<u8> {
    fn from(object: TreeObject) -> Vec<u8> {
        object._raw
    }
}

impl From<TreeObject> for Object {
    fn from(obj: TreeObject) -> Object {
        Object::Tree(obj)
    }
}

impl TryFrom<&[u8]> for TreeObject {
    type Error = InvalidObjectError;
    
    fn try_from(data: &[u8]) -> Result<TreeObject, Self::Error> {
        return Ok(TreeObject{_raw: Vec::from(data)})
    }
}

impl TryFrom<Vec<u8>> for TreeObject {
    type Error = InvalidObjectError;

    fn try_from(data: Vec<u8>) -> Result<TreeObject, Self::Error> {
        return Ok(TreeObject{_raw: data})
    }
}
