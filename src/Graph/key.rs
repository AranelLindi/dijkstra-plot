use crate::Graph::key_type::key_enum::KeyType;

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Key<'a> {
    pub id: &'a str,
    pub attrname: &'a str,
    pub attrtype: KeyType,
    pub default: &'a str,
    //marker: std::marker::PhantomData<&'a &'b()>
}