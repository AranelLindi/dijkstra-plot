use crate::Graph::KeyType::KeyEnum::KeyType;

pub struct Key {
    pub id: str,
    pub attrname: String,
    pub attrtype: KeyType,
    pub value: str
}