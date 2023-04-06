extern crate minidom;

use std::collections::HashSet;
//use std::fmt::format;
use std::fs;
use minidom::Element;

//use std::fs::File;
//use std::io::prelude;
//use std::ops::BitOr;
use crate::Graph::{Graph, Edge, IgraphObject, Key, Node};
use crate::Graph::graph_type::graph_enum::GraphType;
use crate::Graph::key_type::key_enum::KeyType;
use crate::GraphML::key_for::KeyFor;

pub mod key_for {
    #[derive(Debug, Clone, PartialEq)]
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

    // Checks if passed id is already part of a set (and therefore already used).
    fn unused_id(id_set: &HashSet<&str>, id_str: &str) -> bool {
        //let test = id_str.clone();
        if id_str == "" || id_set.contains(id_str) {
            false
        } else {
            true
        }
    }

    // Creates a vector with all data values the graph element has.
    fn create_data_list(ele: &Element, id_str: &str) -> Vec<(String, String)> {
        let mut datas: Vec<(String, String)> = Vec::new();

        for data in ele.children().filter(|&n| n.name() == DATA_NAME) {
            let key = data.attr("key");

            match key {
                Some(key_str) if key_str != "" => {
                    // key exists and is valid
                    let value = data.text();
                    datas.push((key_str.parse().unwrap(), value))
                },
                Some(_) => {
                    // key exists but is empty
                    println!("Missing key id! ({})", id_str);
                    continue;
                }
                None => {
                    // no key attribute was found
                    println!("Missing key attribute! ({})", id_str);
                    continue;
                }
            };
        }

        datas
    }

    fn errfun<T>(error_string: &mut String, error_message: &str) -> Option<T> {
        error_string.push_str(error_message);
        None
    }

    // This function uses all key definitions, checks if they are valid for the current object type (expr) and/or if there is a value that overwrites the default value
    fn key_fusion(keys_map: &Vec<(Key, KeyFor)>, datas: Vec<(String, String)>, expr: impl Fn(&KeyFor) -> bool) -> Vec<Key> {
        let mut keys: Vec<Key> = Vec::new();

        // Check all keys that affect nodes or all elements: (ref keyword is necessary to avoid a move and use a reference binding instead; matches!-macro is useful to make a comparison with enum values)
        for (_, &(ref key, _)) in keys_map.iter().enumerate().filter(|&(_, &(_, ref fortype))| matches!(fortype, _x if expr(fortype))) { // TODO: Gut testen, mögliche Fehlerquelle!
            let mut key_cpy = key.clone();

            // For each key that affects nodes as well as all objects iterate through found data elements whether theres a value which overwrites default value
            for (id, value) in datas.iter() {
                if *id == key.id {
                    // New value was found!
                    key_cpy.value = value.clone();
                    break; // because key id must be unique through whole document, search for other elements is trivial
                }
            }

            // Now: key could added to node keys list:
            keys.push(key_cpy);
        }

        keys
    }

    pub fn create_graph(graphml_path: String) -> Result<Box<Graph<'a>>, Box<String>> {
        // String could be very big therefore put it on heap:
        let mut error_string: Box<String> = Box::new(String::new());

        // Open GraphML file:
        let file = fs::read_to_string(&graphml_path);
        let graphml = file.unwrap_or_else(|_| String::new()); // Contains either content of file or an empty string

        // Contains XML document.
        let xdoc: Element = graphml.parse().unwrap();

        // Ordering of element in GraphML file: keys -> nodes -> edges
        let mut keys_map: Vec<(Key, KeyFor)> = Vec::new();
        let mut nodes: Box<Vec<Node>> = Box::from(Vec::new());
        let mut edges: Box<Vec<Edge>> = Box::from(Vec::new());

        //let mut graph: Box<Graph>;


        // 1. Check first if theres a graphml-root-node:
        if xdoc.name().to_lowercase() == ROOT_NAME {
            // 2. Global graph settings:

            // Graph Id has a special meaning. Because it must already be specified in the constructor, its validity must be checked here immediately!
            let graphid = match xdoc.attr("id") {
                Some(value) => Some(value),
                None => {
                    Self::errfun(&mut error_string, &format!("Missing graph id attribute!\n"))
                }
            };
            if let None = graphid {
                return Err(Box::new("Missing graph id!\n".parse().unwrap()));
            }

            // 2.1 Determine whether theres a global instruction about edges direction (It is not forced to define that globally! So its not necessary to produce error message!)
            let edgedefault = match xdoc.attr("edgedefault") {
                Some(value) => match value {
                    "directed" => Some(GraphType::Directed),
                    "undirected" => Some(GraphType::Undirected),
                    _ => None,
                },
                None => None
            };

            // 3. Keys.
            {
                // Needed variables:
                // HashSet to detect keys with same name:
                let mut keyid: HashSet<&str> = HashSet::new();

                /*
                  Iterate though all keys:
                  All attributes in a Key element must be given! Thats why the code always defines a
                  default value and checks not if an attribute really exists
                 */
                for (i, key) in xdoc.children().filter(|&n| n.name() == KEY_NAME).enumerate() { // TODO: key id must be unique! Implement a checker for that issue!
                    // id.
                    let id = match key.attr("id") {
                        Some(value) if Self::unused_id(&keyid, value) => {
                            // Valid id!
                            keyid.insert(value); // update id set
                            Some(value)
                        },
                        Some(value) => {
                            // Id exists but is used twice!
                            Self::errfun(&mut error_string, &format!("Multiple use of an id! (id:{})\n", value))
                        },
                        _ => {
                            // No id was found!
                            Self::errfun(&mut error_string, &format!("Missing id ! (#{})\n", i))
                        },
                    };

                    // attr.name
                    let attrname = match key.attr("attr.name") {
                        Some(value) => Some(value),
                        None => {
                            Self::errfun(&mut error_string, &format!("No attr.name defined! (#{})\n", i))
                        },
                    };

                    // for
                    let attrfor = match key.attr("for") {
                        Some(value) => match value {
                            "graph" => Some(KeyFor::Graph),
                            "node" => Some(KeyFor::Node),
                            "edge" => Some(KeyFor::Edge),
                            "all" => Some(KeyFor::All),
                            _ => {
                                Self::errfun(&mut error_string, &format!("Invalid value of for attribute! (#{})\n", i))
                            },
                        },
                        None => {
                            Self::errfun(&mut error_string, &format!("Missing for attribute! (#{})\n", i))
                        }
                    };

                    // attr.type
                    let attrtype = match key.attr("attr.type") {
                        Some(value) => match value {
                            "boolean" => Some(KeyType::Boolean),
                            "int" => Some(KeyType::Int),
                            "long" => Some(KeyType::Long),
                            "float" => Some(KeyType::Float),
                            "double" => Some(KeyType::Double),
                            "string" => Some(KeyType::String),
                            _ => {
                                Self::errfun(&mut error_string, &format!("Invalid value for attr.type! (#{})\n", i))
                            },
                        },
                        None => {
                            Self::errfun(&mut error_string, &format!("Missing attr.type attribute! (#{})\n", i))
                        }
                    };

                    // value
                    let value = match key.get_child("default", "") /* TODO: Might be that "" as namespace didn't work as expected! */ {
                        Some(default_node) => Some(default_node.text()),
                        None => {
                            Self::errfun(&mut error_string, &format!("Missing default value! (#{})\n", i))
                        }
                    };

                    // Additional constraints to check:
                    // None

                    // Check if key was correctly parsed and then insert it into key collection.
                    if id.is_some() && attrname.is_some() && attrfor.is_some() && attrtype.is_some() && value.is_some() {
                        keys_map.push((Key {
                            id: id.unwrap().to_string(),
                            attrname: attrname.unwrap().to_string(),
                            attrtype: attrtype.unwrap(),
                            value: value.unwrap() },
                                       attrfor.unwrap()));
                    }
                    else {
                        return Err(Box::from(format!("Found key errors:\n{}", error_string)));
                    }
                }
            }

            // 3.1 Select keys that affect graph object: (for better readability create_data_list was passed here as parameter (and to avoid name conflicts with following variables)
            let keys_graph = Self::key_fusion(&keys_map, Self::create_data_list(&xdoc, "graphml"), |x|(x == &KeyFor::All) || (x == &KeyFor::Graph));

            //if graphid.is_some() {
            //graph = Box::from(Graph::new(graphid.unwrap().to_owned(), Box::from(Vec::new()), Box::from(Vec::new()), Box::from(keys_graph)));
            // 4. Nodes.
            {
                // Needed variables:
                let mut counter: usize = 0;
                // HashSet to detect nodes with same id:
                let mut node_ids: HashSet<&str> = HashSet::new();

                // Iterate through all nodes:
                for (i, node) in xdoc.children().filter(|n| n.name() == NODE_NAME).enumerate() {
                    // Id
                    let id = match node.attr("id") {
                        Some(value) if Self::unused_id(&node_ids, value) => {
                            // Valid node id.
                            node_ids.insert(value); // update node id set
                            Some(value)
                        },
                        Some(value) => {
                            // Multiple used id.
                            Self::errfun(&mut error_string, &format!("Multiple used node id! (id:{})\n", value))
                        },
                        None => {
                            Self::errfun(&mut error_string, &format!("Missing node id! (#{})\n", i))
                        }
                    };

                    if let Some(id_str) = id {
                        // It is not forced that a node must have keys and default value, so this could also be done here first:
                        // Collects all data elements the node might have.
                        let datas = Self::create_data_list(node, id.unwrap());

                        // Check all keys that affect nodes or all elements: (ref keyword is necessary to avoid a move and use a reference binding instead; matches!-macro is useful to make a comparison with enum values)
                        let keys = Self::key_fusion(&keys_map, datas, |x| (x == &KeyFor::All || x == &KeyFor::Node)); // TODO: Gut testen, mögliche Fehlerquelle!

                        // Increase counter:
                        counter += 1;

                        // Here: All information are gathered to create new node:
                        nodes.push(Node::new(id_str.parse().unwrap(), keys, i));
                    }
                    else {
                        return Err(Box::from(format!("Found node errors:\n{}", error_string)));
                    }
                }

                // Additional constraints to check:
                // - There must be at least two nodes
                if counter < 2 {
                    return Err(Box::from(format!("Found node errors:\nGraph must have at least two nodes!\n{}", error_string)));
                }
            }

            //}
            //graph.nodes = nodes;

            // 5. Edges:
            {
                // Needed variables:
                let mut counter: usize = 0;

                // Needed closures:
                // Searches in all known nodes for the one with specified id (At this point: Its ensured that all node id's are unique)
                let node_ref = |src: &str| -> Option<usize> {
                    let opt_node = nodes.iter().find(|&n|n.get_id() == src);
                    match opt_node {
                        Some(val) => Some(val.no()),
                        None => None,
                    }
                };

                // HashSet to detect edges with same id:
                let mut edge_ids: HashSet<&str> = HashSet::new();

                // Iterate through all edges:
                for (i, edge) in xdoc.children().filter(|n| n.name() == EDGE_NAME).enumerate() {
                    // id
                    let id = match edge.attr("id") {
                        Some(value) if Self::unused_id(&edge_ids, value) => Some(value),
                        Some(value) => {
                            Self::errfun(&mut error_string, &format!("Multiple used edge id! (id:{})\n", value))
                        },
                        None => {
                            Self::errfun(&mut error_string, &format!("Missing edge id attribute! (#{})\n", i))
                        }
                    };

                    // weight:
                    let weight = match edge.attr("weight") {
                        Some(value) => {
                            let _weight = value.parse::<u32>();
                            match _weight {
                                Ok(val) => Some(val),
                                Err(_) => {
                                    Self::errfun(&mut error_string, &format!("Invalid value for edge weight! (#{})\n", i))
                                },
                            }},
                        None => {
                            Self::errfun(&mut error_string, &format!("Missing edge weight attribute! (#{})\n", i))
                        },
                    };

                    // directed:
                    let directed = match edge.attr("directed") {
                        Some(value) => match value {
                            "directed" => Some(GraphType::Directed),
                            "undirected" => Some(GraphType::Undirected),
                            _ => {
                                // Invalid value in attribute!
                                Self::errfun(&mut error_string, &format!("Invalid value for edge directed attribute! (#{})\n", i))
                            },
                        },
                        None => match &edgedefault /* An immutable reference is used here otherwise a move is assumed! */ {
                            Some(default) => Some(default.clone()),
                            None => {
                                // Attribute was not found!
                                Self::errfun(&mut error_string, &format!("Neither default edge direction nor directed attribute found! (#{})\n", i))
                            },
                        }
                    };

                    // source:
                    let source = match edge.attr("source") {
                        Some(value ) => {
                            if let Some(reference) = node_ref(value) {
                                Some(reference)
                            }
                            else {
                                // Undefined reference: Value in attribute could not mapped to a real node id!
                                Self::errfun(&mut error_string, &format!("Undefined source reference! (#{})\n", i))
                            }
                        },
                        None => {
                            // No attribute found!
                            Self::errfun(&mut error_string, &format!("Missing edge source attribute! (#{})\n", i))
                        }
                    };

                    // destination:
                    let destination = match edge.attr("target") {
                        Some(value ) => {
                            if let Some(reference) = node_ref(value) {
                                Some(reference)
                            }
                            else {
                                // Undefined reference: Value in attribute could not mapped to a real node id!
                                Self::errfun(&mut error_string, &format!("Undefined target reference! (#{})\n", i))
                            }
                        },
                        None => {
                            // No attribute found!
                            Self::errfun(&mut error_string, &format!("Missing edge target attribute! (#{})\n", i))
                        }
                    };

                    //rel.push((source, destination));
                    //graph.nodes = nodes;

                    if id.is_some() && weight.is_some() && directed.is_some() && source.is_some() && destination.is_some() {
                        // It is not forced that a node must have keys and default value, so this could also be done here first:
                        // Collects all data elements the node might have and iterates through all data elements of the node.
                        let datas = Self::create_data_list(edge, id.unwrap());

                        counter += 1;

                        // Check all keys that affect nodes or all elements: (ref keyword is necessary to avoid a move and use a reference binding instead; matches!-macro is useful to make a comparison with enum values)
                        let keys = Self::key_fusion(&keys_map, datas, |x| (x == &KeyFor::All) | (x == &KeyFor::Edge));

                        // Here: All information is available to create new edge:
                        edges.push(Edge::new(id.unwrap().to_string(), weight.unwrap(), directed.unwrap(), &nodes[source.unwrap()], &nodes[destination.unwrap()], keys));
                    }
                    else {
                        return Err(Box::from(format!("Found edge errors:\n{}", error_string)));
                    }
                }

                //graph.nodes = nodes;
                //graph.edges = edges;


                let mut graph2 = Box::from(Graph::new(format!("test"), nodes.clone(), edges.clone(), Box::from(keys_graph.clone())));
                return Ok(graph2);

                // Additional constraints to check:
                // - There must be at least one edge
                if counter == 0 {
                    return Err(Box::from(format!("Found node errors:\nGraph must have at least one edge!\n{}", error_string)));
                }
            }



            if graphid.is_some() {
                //graph.edges = edges;
                // Here: All information is available to create new graph:
                //return Ok(graph);
                return Ok(Box::from(Graph::new(format!("test"), Box::from(Vec::new()), Box::from(Vec::new()), Box::from(Vec::new()))));
                //return Ok(Box::from(Graph::new(format!("test"), nodes.clone(), Box::from(Vec::new()), Box::from(Vec::new()))));
            }
            else {
                return Err(Box::from(format!("Found graph errors:\nGraph must have an id!\n{}", error_string)));
            }
        }
        else {
            // No root element was found -> throw exception!
            return Err(Box::from("No root graphml-node was found!\n".to_string()));
        }
    }
}