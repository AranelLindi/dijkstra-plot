use crate::Graph::graphtype::graphtype;
use crate::Graph::IGraphObject::IGraphObject;
use crate::Graph::Key::Key;
use crate::Graph::Node::Node;

pub struct Edge<'a> {
    id: str,
    pub weight: u32,
    pub etype: graphtype,
    pub source: &'a Node,
    pub dest: &'a Node,
    keys: Vec<Key>,
}

impl IGraphObject for Edge {
    fn getID(&self) -> &str {
        &self.id
    }

    fn getKeys(&self) -> &Vec<Key> {
        &self.keys
    }
}
