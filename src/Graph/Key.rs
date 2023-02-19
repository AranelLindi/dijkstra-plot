mod enum_KeyType {
    pub use enum_KeyType::KeyType;
}

struct Key {
    pub id: String,
    pub attrname: String,
    pub attrtype: enum_KeyType::KeyType,
    pub value: String,
}