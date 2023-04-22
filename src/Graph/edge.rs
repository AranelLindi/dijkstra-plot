//use std::cell::RefCell;
use std::rc::Rc;
use crate::Node;
use crate::Graph::graph_type::graph_enum::GraphType as GraphType;
use crate::Graph::key::Key as Key;
use crate::Graph::igraph_object::IgraphObject;

#[derive(Clone)]
pub struct Edge<'a> {
    id: &'a str,
    weight: u32,
    etype: GraphType,
    source: Rc<Node<'a>>,
    dest: Rc<Node<'a>>,
    keys: Vec<Key<'a>>,
}

impl<'a> Edge<'a> {
    pub fn new(id: &'a str, weight: u32, etype: GraphType, source: Rc<Node<'a>>, dest: Rc<Node<'a>>, keys: Vec<Key<'a>>) -> Self {
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
    pub fn source(&self) -> Rc<Node> {
        self.source.clone()
    }
    pub fn dest(&self) -> Rc<Node> {
        self.dest.clone()
    }
}


impl<'a> IgraphObject<'a> for Edge<'a> {
    fn get_id(&'a self) -> &'a str {
        self.id
    }

    fn get_keys(&mut self) -> &'a mut Vec<Key> {
        & mut self.keys
    }

    fn set_keys(&'a mut self, keys: Vec<Key<'a>>) {
        self.keys = keys;//.to_vec();
    }
}
