use crate::Graph::key_type::key_enum::KeyType;

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Key {
    pub id: String,
    pub attrname: String,
    pub attrtype: KeyType,
    pub value: String,
    //marker: std::marker::PhantomData<&'a &'b()>
}