use crate::Graph::Key::Key;
use crate::Graph::IGraphObject::IGraphObject; //{Key, IGraphObject};

pub struct Node {
    id: String,
    keys: Vec<Key>,
    no: u32
}

impl IGraphObject for Node {
    fn getID(&self) -> &str {
        &self.id
    }

    fn getKeys(&self) -> &Vec<Key> {
        &self.keys
    }
}