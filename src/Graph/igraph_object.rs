use crate::Graph::key::Key;

pub trait IgraphObject<'a> {
    fn get_id(&'a self) -> &'a str;
    fn get_keys(&'a mut self) -> &'a mut Vec<Key<'a>>;
    fn set_keys(&'a mut self, keys: Vec<Key<'a>>);
}