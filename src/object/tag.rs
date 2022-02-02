use super::*;

#[derive(Debug, Clone)]
pub struct TagObject {
    _raw: Box<[u8]>,
}

impl GitObject for TagObject {
    const TYPE: &'static str = "tag";
}

impl Serializable for TagObject {
    fn serialize(&self) -> &[u8] {
        unimplemented!()
    }

    fn deserialize(data: &[u8]) -> Self {
        return Self {
            _raw: Box::from(data),
        };
    }
}

impl From<TagObject> for Object {
    fn from(obj: TagObject) -> Object {
        Object::Tag(obj)
    }
}
