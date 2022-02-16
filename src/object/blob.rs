use super::*;

#[derive(Debug, Clone)]
pub struct BlobObject {
    _raw: Vec<u8>,
}

impl GitObject for BlobObject {
    const TYPE: &'static str = "blob";
}

impl From<BlobObject> for Vec<u8> {
    fn from(object: BlobObject) -> Vec<u8> {
        return object._raw;
    }
}

impl From<BlobObject> for Object {
    fn from(obj: BlobObject) -> Object {
        Object::Blob(obj)
    }
}

impl TryFrom<&[u8]> for BlobObject {
    type Error = InvalidObjectError;
    
    fn try_from(data: &[u8]) -> Result<BlobObject, Self::Error> {
        return Ok(BlobObject{_raw: Vec::from(data)})
    }
}

impl TryFrom<Vec<u8>> for BlobObject {
    type Error = InvalidObjectError;
    
    fn try_from(data: Vec<u8>) -> Result<BlobObject, Self::Error> {
        return Ok(BlobObject{_raw: data})
    }
}