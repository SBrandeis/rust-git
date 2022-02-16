use super::*;

#[derive(Debug, Clone)]
pub struct TagObject {
    _raw: Vec<u8>,
}

impl GitObject for TagObject {
    const TYPE: &'static str = "tag";
}


impl From<TagObject> for Object {
    fn from(obj: TagObject) -> Object {
        Object::Tag(obj)
    }
}

impl From<TagObject> for Vec<u8> {
    fn from(object: TagObject) -> Vec<u8> {
        object._raw
    }
}

impl TryFrom<&[u8]> for TagObject {
    type Error = InvalidObjectError;

    fn try_from(data: &[u8]) -> Result<TagObject, Self::Error> {
        return Ok(TagObject{_raw: Vec::from(data)})
    }
}

impl TryFrom<Vec<u8>> for TagObject {
    type Error = InvalidObjectError;

    fn try_from(data: Vec<u8>) -> Result<TagObject, Self::Error> {
        return Ok(TagObject{_raw: data})
    }
}
