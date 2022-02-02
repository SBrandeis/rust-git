use super::*;

#[derive(Debug, Clone)]
pub struct BlobObject {
    _raw: Box<[u8]>,
}

impl GitObject for BlobObject {
    const TYPE: &'static str = "blob";
}

impl Serializable for BlobObject {
    fn serialize(&self) -> &[u8] {
        &self._raw
    }

    fn deserialize(data: &[u8]) -> Self {
        Self {
            _raw: Box::from(data),
        }
    }
}

impl From<BlobObject> for Object {
    fn from(obj: BlobObject) -> Object {
        Object::Blob(obj)
    }
}
