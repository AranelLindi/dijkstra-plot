extern crate minidom;

use std::fs;
use minidom::Element;

use std::fs::File;
use std::io::prelude;
use crate::Graph::{Edge, Key, Node};
use crate::Graph::graph_type::graph_enum::GraphType;
use crate::Graph::key_type::key_enum::KeyType;
use crate::GraphML::key_for::KeyFor;
use crate::GraphML::key_for::KeyFor::Graph;

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

        // Contains XML document.
        let xdoc: Element = graphml.parse().unwrap();

        // Ordering of element in GraphML file: keys -> nodes -> edges
        let mut keys_map: Vec<(Key, KeyFor)> = Vec::new();
        let mut nodes: Vec<Node> = Vec::new();
        let mut edges: Vec<Edge> = Vec::new();

        // Check first if theres a graphml-root-node:
        if xdoc.name().to_lowercase() == ROOT_NAME {
            // Determine whether theres a global instruction about edges direction:
            let edgedefault = match xdoc.attr("edgedefault").unwrap_or_else(|| "undirected") {
                "directed" => GraphType::Directed,
                "undirected" | _ => GraphType::Undirected,
            };

            // Iterate though all keys:
            for key in xdoc.children().filter(|n| n.name() == KEY_NAME) { // TODO: key id must be unique! Implement a checker for that issue!
                // id.
                let id = key.attr("id").unwrap_or_default().to_string();
                // attr.name
                let attrname = key.attr("attr.name").unwrap_or_default().to_string();
                // for
                let attrfor = match key.attr("for").unwrap_or_else(|| "all") {
                    "graph" => KeyFor::Graph,
                    "node" => KeyFor::Node,
                    "edge" => KeyFor::Edge,
                    "all" | _ => KeyFor::All,
                };
                // type
                let attrtype = match key.attr("attr.type").unwrap_or_else(|| "string") {
                    "boolean" => KeyType::Boolean,
                    "int" => KeyType::Int,
                    "long" => KeyType::Long,
                    "Float" => KeyType::Float,
                    "Double" => KeyType::Double,
                    "String" | _ => KeyType::String,
                };
                // value
                let value = match key.get_child("default", "") /* TODO: Might be that "" as namespace didn' work as expected! */ {
                    Some(default_node) => default_node.text(),
                    None => "ERR".to_string(),
                };

                // Add key to vector:
                keys_map.push((Key{id, attrname, attrtype, default: value }, attrfor));
            }

            // Iterate through all nodes:
            for (i, node) in xdoc.children().filter(|n| n.name() == NODE_NAME).enumerate() {
                let id = node.attr("id").unwrap_or_default();

                // Keys of the current node:
                let mut keys: Vec<Key> = Vec::new();

                // Collects all data elements the node might have.
                let mut datas: Vec<(String, String)> = Vec::new();

                // Iterate through all data elements of the node.
                for data in node.children().filter(|n| n.name() == DATA_NAME) {
                    let key = data.attr("key").unwrap_or_default().to_string();
                    let value = data.text();

                    // Check if theres an error by no key name was found jump to next iteration...
                    if key == "" {continue}

                    // ... otherwise add tuple to datas vector.
                    datas.push((key, value));
                }

                // Check all keys that affect nodes or all elements: (ref keyword is necessary to avoid a move and use a reference binding instead; matches!-macro is useful to make a comparison with enum values)
                for (_, &(ref key, _)) in keys_map.iter().enumerate().filter(|&(_, &(_, ref _for))| matches!(_for, KeyFor::All | KeyFor::Node)) {
                    let mut key_cpy = key.clone();

                    // For each key that affects nodes as well as all objects iterate through found data elements whether theres a value which overwrites default value
                    for (id, value) in datas.iter() {
                        if *id == key.id {
                            // New value was found!
                            key_cpy.default = value.clone();
                            break; // because key id must be unique through whole document, search for other elements is trivial
                        }
                    }

                    // Now: key could added to node keys list:
                    keys.push(key_cpy);
                }

                // Here: All information are gathered to create new node:
                nodes.push(Node::new(id.parse().unwrap(), keys, i as u32));
            }
        }
        else {
            // No root element was found -> fire error!
        }
    }
}