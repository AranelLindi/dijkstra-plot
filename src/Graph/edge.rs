use crate::Graph::graphtype::Graphtype;
use crate::Graph::igraph_object::IgraphObject;
use crate::Graph::key::Key;
use crate::Graph::node::Node;

pub struct Edge<'a, 'b> {
    id: &'a str,
    pub weight: u32,
    pub etype: graphtype,
    pub source: &'b Node<'a>,
    pub dest: &'b Node<'a>,
    keys: Vec<key>,
}

impl<'a, 'b> IgraphObject for Edge<'a, 'b> {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_keys(&self) -> &Vec<key> {
        &self.keys
    }
}
