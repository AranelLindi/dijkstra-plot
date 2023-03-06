use crate::Node;
use crate::Graph::graph_type::graph_enum::GraphType as GraphType;
use crate::Graph::key::Key as Key;
use crate::Graph::igraph_object::IgraphObject;

#[derive(Clone)]
pub struct Edge<'a> {
    id: String,
    weight: u32,
    etype: GraphType,
    source: &'a Node,
    dest: &'a Node,
    keys: Vec<Key>,
}

impl<'a> Edge<'a> {
    pub fn new(id: String, weight: u32, etype: GraphType, source: &'a Node, dest: &'a Node, keys: Vec<Key>) -> Self {
        Self {
            id,
            weight,
            etype,
            source,
            dest,
            keys
        }
    }
    pub fn weight(&self) -> u32 {
        self.weight
    }
    pub fn etype(&self) -> &GraphType {
        &self.etype
    }
    pub fn source(&self) -> &'a Node {
        self.source
    }
    pub fn dest(&self) -> &'a Node {
        self.dest
    }
}


impl<'a> IgraphObject<'a> for Edge<'a> {
    fn get_id(&'a self) -> &'a str {
        self.id.as_str()
    }

    fn get_keys(&mut self) -> &mut Vec<Key> {
        & mut self.keys
    }

    fn set_keys(&mut self, keys: Vec<Key>) {
        self.keys = keys;//.to_vec();
    }
}
