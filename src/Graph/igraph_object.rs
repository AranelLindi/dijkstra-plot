use crate::Graph::key::Key;

pub trait IgraphObject {
    fn get_id(&self) -> &str;
    fn get_keys(&self) -> &Vec<Key>;
}