use super::*;

#[derive(Debug, Clone)]
pub struct TreeObject {
    _raw: Box<[u8]>,
}

impl GitObject for TreeObject {
    const TYPE: &'static str = "tree";
}

impl Serializable for TreeObject {
    fn serialize(&self) -> &[u8] {
        unimplemented!()
    }

    fn deserialize(data: &[u8]) -> Self {
        return Self {
            _raw: Box::from(data),
        };
    }
}

impl From<TreeObject> for Object {
    fn from(obj: TreeObject) -> Object {
        Object::Tree(obj)
    }
}
