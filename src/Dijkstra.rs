//mod Graph;

use crate::Graph::{Node, Edge, Key, KeyType};
//use crate::Graph::Node;

pub struct Dijkstra {}

impl Dijkstra {
    fn init(graph: &Graph, start: &Node) {
        for (_, e) in graph.nodes.iter().enumerate() {
            Graph.addKey(e, Key {id: "dijkstra1", attrname: "abstand", keytype: KeyType::int, value="inf"});
        }
    }

    pub fn run(graph: &Graph, start: &Node, dest: &Node) {
        // TODO
    }
}