use crate::Graph::key::Key;

pub trait IgraphObject<'a> {
    fn get_id(&'a self) -> &'a str;
    fn get_keys(&mut self) -> &mut Vec<Key>;
    fn set_keys(&mut self, keys: Vec<Key>);
}