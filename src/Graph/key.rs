use crate::Graph::key_type::key_enum::KeyType;

pub struct Key {
    pub id: str,
    pub attrname: String,
    pub attrtype: KeyType,
    pub value: str
}