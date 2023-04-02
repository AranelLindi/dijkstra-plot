extern crate minidom;

use std::fs;
use minidom::Element;

use std::fs::File;
use std::io::prelude;
use crate::Graph::{Edge, Key, Node};
use crate::Graph::key_type::key_enum::KeyType;
use crate::GraphML::key_for::KeyFor;

pub mod key_for {
    #[derive(Clone)]
    pub enum KeyFor {
        Graph,
        Node,
        Edge,
        All,
    }
}


struct GraphML<'a> {
    marker: std::marker::PhantomData<&'a()>
}

// Constants used in GraphML files:
const ROOT_NAME: &str = "graphml";
const NODE_NAME: &str = "node";
const EDGE_NAME: &str = "edge";
const DATA_NAME: &str = "data";
const KEY_NAME: &str = "key";

impl<'a> GraphML<'a> {
    fn addKeyToNode(node: &'a mut Node, key: Key) {}
    fn addKeyToNodes(nodes: &'a mut Vec<Node>, key: Key) {}
    fn addKeyToEdge(edge: &'a mut Edge, key: Key) {}
    fn addKeyToEdges(edges: &'a mut Vec<Edge>, key: Key) {}


    pub fn createGraph(graphml_path: String) {
        // Open GraphML file:
        let file = fs::read_to_string(&graphml_path);
        let graphml = file.unwrap_or_else(|_| String::new()); // Contains either content of file or an empty string

        let xdoc: Element = graphml.parse().unwrap();

        let mut keys: Vec<(Key, KeyFor)> = Vec::new();
        let mut nodes: Vec<Node> = Vec::new();
        let mut edges: Vec<Edge> = Vec::new();

        if xdoc.name().to_lowercase() == ROOT_NAME {
            for child in xdoc.children().filter(|n| n.name() == KEY_NAME) {
                // id.
                let id = child.attr("id").unwrap_or_default().to_string();
                // attr.name
                let attrname = child.attr("attr.name").unwrap_or_default().to_string();
                // for
                let attrfor = match child.attr("for").unwrap_or_else(|| "all") {
                    "graph" => KeyFor::Graph,
                    "node" => KeyFor::Node,
                    "edge" => KeyFor::Edge,
                    "all" | _ => KeyFor::All,
                };
                // type
                let attrtype = match child.attr("attr.type").unwrap_or_else(|| "string") {
                    "boolean" => KeyType::Boolean,
                    "int" => KeyType::Int,
                    "long" => KeyType::Long,
                    "Float" => KeyType::Float,
                    "Double" => KeyType::Double,
                    "String" | _ => KeyType::String,
                };
                // value
                let value = match child.get_child("default", "") /* TODO: Might be that "" as namespace didn' work as expected! */ {
                    Some(default_node) => default_node.text(),
                    None => "ERR".to_string(),
                };

                // Add key to vector:
                keys.push((Key{id, attrname, attrtype, value}, attrfor));
            }
        }
        else {
            // No root element was found -> fire error!
        }
    }
}