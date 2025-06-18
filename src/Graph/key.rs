use crate::Graph::key_type::key_enum::KeyType;

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Key {
    pub id: String,
    pub attrname: String,
    pub attrtype: KeyType,
    pub default: String, // TODO: Better name would be "default" to better correspond to default graphml node
    //marker: std::marker::PhantomData<&'a &'b()>
}