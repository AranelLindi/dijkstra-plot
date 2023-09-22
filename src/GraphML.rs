use xml::reader::{EventReader, XmlEvent};

use std::io::BufRead;

use std::collections::HashSet;
//use minidom::Element;
use std::fs::{File, read_to_string};
use std::io::BufReader;
use std::rc::Rc;
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;

//use quick_xml::events::Event;
//use quick_xml::reader::Reader;

use crate::Graph::graph_type::graph_enum::GraphType;
use crate::Graph::key_type::key_enum::KeyType;
use crate::Graph::{Edge, Graph, IgraphObject, Key, Node};
use crate::GraphML::key_for::KeyFor;

pub mod key_for {
    #[derive(Debug, Clone, PartialEq)] // TODO: Brauche ich hier alle?
    pub enum KeyFor {
        Graph,
        Node,
        Edge,
        All,
    }
}

pub struct GraphML<'a> {
    marker: std::marker::PhantomData<&'a ()>,
}

// Constants used in GraphML files:
const ROOT_NAME: &str = "graphml";
const GRAPH_NAME: &str = "graph";
const NODE_NAME: &str = "node";
const EDGE_NAME: &str = "edge";
const DATA_NAME: &str = "data";
const KEY_NAME: &str = "key";
// TODO: Hier alle Strings definieren! Spart Speicherplatz!


#[derive(PartialEq)]
enum GraphMLDocStatus {
    Null,
    RootNode,
    KeyNode,
    GraphNode,
    NodeNode,
    EdgeNode
}

impl<'a> GraphML<'a> {
    // Checks if passed id is already part of a set (and therefore already used).
    fn unused_id(id_set: &HashSet<&str>, id_str: &str) -> bool {
        if id_str == "" || id_set.contains(id_str) {
            false // already used or empty
        } else {
            true // not used so far
        }
    }

    // Creates a vector with all data values the graph element has. (E.g. <ele><data key="xyz">[value]</key>...</ele>)
    fn create_data_list(ele: &Element, id_str: &str) -> Vec<(&'a str, &'a str)> {
        // Vector of found (key_id, key_value):
        let mut datas: Vec<(&'a str, &'a str)> = Vec::new();

        // Iterates through all data nodes the element has:
        for data in ele.children().filter(|&n| n.name() == DATA_NAME) {
            let key = data.attr("key");

            match key {
                Some(key_value) if key_value != "" => {
                    // key exists and is valid!

                    // &str slice is boxed to have a longer lifetime then just function scope!
                    let id = Box::leak(id_str.to_string().into_boxed_str()) as &'a str;
                    let value = Box::leak(key_value.to_string().into_boxed_str()) as &'a str;

                    datas.push((id, value))
                }
                Some(_) => {
                    // key exists but is empty!
                    println!("Missing key id! ({})", id_str);
                    continue;
                }
                None => {
                    // no key attribute was found (no assignment possible)
                    println!("Missing key attribute! ({})", id_str);
                    continue;
                }
            };
        }

        datas
    }

    // Error function: Appends passed error message to global string and returns None (often used, therefore swapped out as function!)
    fn errfun<T>(error_string: &mut String, error_message: &str) -> Option<T> {
        error_string.push_str(error_message); // appends message
        None
    }

    // This function uses all key definitions (keys_map), checks if they are valid for the current object type (expr) and/or if there is a value that overwrites the default value (datas)
    fn key_fusion(keys_map: &Vec<(Key<'a>, KeyFor)>, datas: Vec<(&'a str, &'a str)>, expr: impl Fn(&KeyFor) -> bool) -> Vec<Key<'a>> {
        // Will contain all keys that affect the current object:
        let mut keys: Vec<Key> = Vec::new();

        // Check all keys that affect in expr specified elements: (ref keyword is necessary to avoid a move and use a reference binding instead; matches!-macro is useful to make a comparison with enum values)
        for (_, &(ref key, _)) in keys_map
            .iter()
            .enumerate()
            .filter(|&(_, &(_, ref for_type))| matches!(for_type, _x if expr(for_type)))
        {
            // Copy! Otherwise is a move assumed but it must be part of keys_map.
            let mut key_cpy = key.clone();

            // For each key that affects the current object, the data elements found are checked to see if there is a value that overrides the default value
            for (id, value) in datas.iter() {
                if *id == key.id {
                    // New value was found - replace!
                    key_cpy.default = value.clone();
                    break; // because key id must be unique through whole document (at this point ensured), search for further elements in datas is trivial!
                }
            }

            // Now: key could added to node keys list:
            keys.push(key_cpy);
        }

        keys
    }

    // Creates a graph object out of file path to GraphML (XML)-file.
    pub fn create_graph(graphml_path: String) -> Result<Graph<'a>, Box<String>> {
        // Contains error messages. (String could be very big therefore put it on heap):
        let mut error_string: Box<String> = Box::new(String::new());

        let mut status = GraphMLDocStatus::Null;

        let mut xml_reader = EventReader::from_str(&*graphml_path);

        let mut keys: Vec<Key> = Vec::new();

        for event in xml_reader {
            match event {
                Ok(obj) => {
                    match obj {
                        XmlEvent::StartDocument { .. } => {}
                        XmlEvent::EndDocument => {}
                        XmlEvent::ProcessingInstruction { .. } => {}
                        XmlEvent::StartElement { name, attributes, .. } => {
                            match name.local_name.to_lowercase().as_str() {
                                "graphml" => {
                                    if status == GraphMLDocStatus::Null { // sicher
                                        status = GraphMLDocStatus::RootNode;
                                        // Placeholder: No attributes are needed here but necessary to guarantee correct sequence
                                    }
                                    else {
                                        // Error! Root Node must be first node in document!
                                    }

                                },
                                "key" => {
                                    if status == GraphMLDocStatus::KeyNode || status == GraphMLDocStatus::RootNode { // nicht so ganz sicher (muss key hier echt nicht dabei sein?)
                                        status = GraphMLDocStatus::KeyNode;

                                        let mut attr_id: Option<String> = Option::None;
                                        let mut attr_for: Option<String> = Option::None;
                                        let mut attr_name: Option<String> = Option::None;
                                        let mut attr_type: Option<KeyType> = Option::None;
                                        let mut default: Option<String> = Option::None;


                                        // Unpack key element:
                                        // 1. Attributes:
                                        for attribute in attributes {
                                            match attribute.name.local_name.to_lowercase().as_str() {
                                                "id" => {
                                                    attr_id = Option::Some(attribute.value.to_lowercase())
                                                },
                                                "for" => {
                                                    match attribute.value.to_lowercase().as_str() {
                                                        "all" | "node" | "edge" => {
                                                            attr_for = Option::Some(attribute.value.to_lowercase())
                                                        },
                                                        _ => {
                                                            // Error! Invalid value!
                                                        }
                                                    }
                                                },
                                                "attr.name" => {
                                                    attr_name = Option::Some(attribute.value.to_lowercase())
                                                },
                                                "attr.type" => {
                                                    match attribute.value.to_lowercase().as_str() {
                                                        "boolean" => {
                                                            attr_type = Option::Some(KeyType::Boolean);
                                                        },
                                                        "int" => {
                                                            attr_type = Option::Some(KeyType::Int)
                                                        },
                                                        "long" => {
                                                            attr_type = Option::Some(KeyType::Long)
                                                        },
                                                        "float" => {
                                                            attr_type = Option::Some(KeyType::Float)
                                                        },
                                                        "double" => {
                                                            attr_type = Option::Some(KeyType::Double)
                                                        },
                                                        "string" => {
                                                            attr_type = Option::Some(KeyType::String)
                                                        },
                                                        _ => {
                                                            // Error! Invalid value
                                                        }
                                                    }
                                                },
                                                _ => { continue; /* Actually it isn't allowed to have non-graphml attributes but I make an exception here */ }
                                            }
                                        }

                                        // Optional: child node with default value!
                                        let mut has_default = false;

                                        if let Ok(event) = xml_reader.next() {
                                            match event {
                                                XmlEvent::StartElement { name, .. } => {
                                                    match name.local_name.to_lowercase().as_str() {
                                                        "default" => {
                                                            has_default = true;
                                                        },
                                                        _ => {
                                                            // Error! Invalid node name
                                                        }
                                                    }
                                                },
                                                XmlEvent::Characters(text) => {
                                                    if has_default {
                                                        default = Some(text);
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }

                                        // All information gathered at this point!

                                        // Check if variables are filled with valid information
                                        if !(attr_id == Option::None && attr_for == Option::None && attr_name == Option::None && attr_type == Option::None) {
                                            // Valid object
                                            let mut key: Key;

                                            // TODO: Stopped here! Need to assign the values (like attr_id) to the members of the Key object and then add it (with keys.push()) to the vector. After that continue code the other nodes
                                        }
                                    }
                                    else {
                                        // Error! Wrong sequence! Key node was detected in invalid order!
                                    }

                                },
                                "graph" => {
                                    //key_processed = true; // for the case no keys were defined its necessary to assign the variable here to make sure that the rest of the document can be read in

                                    if status == GraphMLDocStatus::RootNode || status == GraphMLDocStatus::KeyNode { // nicht so ganz sicher
                                        status = GraphMLDocStatus::GraphNode;

                                        // Attribute lesen
                                    }
                                    else {
                                        // Error!
                                    }
                                },
                                "node" => {
                                    if status == GraphMLDocStatus::NodeNode || status == GraphMLDocStatus::GraphNode {

                                    }
                                    else {
                                        // Error!
                                    }
                                },
                                "edge" => {
                                    //node_processed = true; // from now on node-nodes aren't allowed anymore
                                    //edge_processed = true; // from now on only edge-nodes are allowed

                                    if status == GraphMLDocStatus::EdgeNode || status == GraphMLDocStatus::NodeNode {

                                    }
                                    else {
                                        // Error!
                                    }
                                },
                                _ => {
                                    // Unspecified node name -> Error!
                                }
                            }
                        }
                        XmlEvent::EndElement { name } => {
                            match name.local_name.to_lowercase().as_str() {
                                "graphml" => { break; },
                                _ => {}
                            }
                        }
                        XmlEvent::CData(_) => {}
                        XmlEvent::Comment(_) => {}
                        XmlEvent::Characters(_) => {}
                        XmlEvent::Whitespace(_) => {}
                    }
                }
                Err(_) => {}
            }
        }


        let mut txt = Vec::new();
        let mut buf = Vec::new();

        if let Some(graph)


        let mut reader = from_str(read_to_string(graphml_path).ex)
        reader.trim_text(true);

        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {


                    // https://github.com/tafia/quick-xml
                    // https://crates.io/crates/quick-xml

                    match e.name().as_ref() {

                    }
                }
            }
        }




        // (Ordering of elements in GraphML file: keys -> nodes -> edges)

        // 1. Check first if theres a graphml-root-node:
        if xdoc.name().to_lowercase() == ROOT_NAME {

            // 2. Keys.
            // Will contain all key definitions and which elements they are affecting:
            let mut keys_map: Vec<(Key, KeyFor)> = Vec::new();

            {
                // 2.1 Needed temp variables:
                // HashSet to detect keys with same name to ensure that no id is used more than once:
                let mut keyid: HashSet<&str> = HashSet::new();

                /* 2.2 Iterates through all key elements. Each key element must have the following attributes/nodes:
                Attributes:
                - id (String)
                - for (KeyFor)
                - attr.name (String)
                - attr.type (KeyTpe)
                Nodes:
                - default (depending on attr.type but interpreted as String)

                If any attribute/node is missing, key cannot be generated!
                */
                for (i, key) in xdoc
                    .children()
                    .filter(|&n| n.name() == KEY_NAME)
                    .enumerate()
                {
                    // 2.2.1 id.
                    let id = match key.attr("id") {
                        Some(id_value) if Self::unused_id(&keyid, id_value) => {
                            // Valid id!

                            // Box the &str to make sure it does live as long as the lifetime parameter 'a lives.
                            let value = Box::leak(id_value.to_string().into_boxed_str()) as &'a str;

                            keyid.insert(value); // update id set
                            Some(value)
                        }
                        Some(id_value) => {
                            // Id exists but is used twice!
                            Self::errfun(
                                &mut error_string,
                                &format!("Multiple use of an id! (id:{})\n", id_value),
                            )
                        }
                        _ => {
                            // No id was found!
                            Self::errfun(&mut error_string, &format!("Missing id ! (#{})\n", i))
                        }
                    };

                    // 2.2.2 for
                    let attrfor = match key.attr("for") {
                        Some(attrfor_value) => match attrfor_value {
                            "graph" => Some(KeyFor::Graph),
                            "node" => Some(KeyFor::Node),
                            "edge" => Some(KeyFor::Edge),
                            "all" => Some(KeyFor::All),
                            _ => Self::errfun(
                                &mut error_string,
                                &format!("Invalid value of for attribute! (#{})\n", i),
                            ),
                        },
                        None => Self::errfun(
                            &mut error_string,
                            &format!("Missing for attribute! (#{})\n", i),
                        ),
                    };

                    // 2.2.3 attr.name
                    let attrname = match key.attr("attr.name") {
                        Some(attrname_value) => {
                            // Box the &str to make sure that it lives as long as 'a lifetime parameter lives.
                            let value = Box::leak(attrname_value.to_string().into_boxed_str()) as &'a str;
                            Some(value)},
                        None => Self::errfun(
                            &mut error_string,
                            &format!("No attr.name defined! (#{})\n", i),
                        ),
                    };

                    // 2.2.4 attr.type
                    let attrtype = match key.attr("attr.type") {
                        Some(attrtype_value) => match attrtype_value {
                            "boolean" => Some(KeyType::Boolean),
                            "int" => Some(KeyType::Int),
                            "long" => Some(KeyType::Long),
                            "float" => Some(KeyType::Float),
                            "double" => Some(KeyType::Double),
                            "string" => Some(KeyType::String),
                            _ => Self::errfun(
                                &mut error_string,
                                &format!("Invalid value for attr.type! (#{})\n", i),
                            ),
                        },
                        None => Self::errfun(
                            &mut error_string,
                            &format!("Missing attr.type attribute! (#{})\n", i),
                        ),
                    };

                    // 2.2.5 value (A default value must be defined! Otherwise must be secured that each element which implements that key has an own value for it (which is to complicated))
                    let value = match key.get_child("default", "") /* TODO: Might be that "" as namespace didn't work as expected! */ {
                        Some(default_node) => {
                            // Box the &str to make sure that it lives as long as 'a lifetime parameter lives.
                            let value = Box::leak(default_node.text().into_boxed_str()) as &'a str;
                            Some(value) /* inner text of node */},
                        None => {
                            Self::errfun(&mut error_string, &format!("Missing default value! (#{})\n", i))
                        }
                    };

                    // 2.2.6 Additional constraints to check:
                    // None

                    // 2.2.7 Check if key was correctly parsed and then insert it into key collection.
                    if id.is_some() && attrfor.is_some() && attrname.is_some() && attrtype.is_some() && value.is_some() {
                        keys_map.push((
                            Key {
                                id: id.unwrap(), /* if there occurs an error then maybe because the string isn't copied! Try .clone() then! */
                                attrname: attrname.unwrap(),
                                attrtype: attrtype.unwrap(),
                                default: value.unwrap(),
                            },
                            attrfor.unwrap(),
                        ));
                    } else {
                        return Err(Box::from(format!("Found key errors:\n{}", error_string)));
                    }
                }
            }

            // 3. Graph (node).
            // Following variables will contain all information which is necessary to create both Node and Edge objects (needed until end of function!):
            let mut nodes: Vec<(&'a str, Vec<Key>, usize)> = Vec::new(); // vector tuple: ([id], Vec[keys], [no])
            let mut edges: Vec<(&'a str, u32, GraphType, usize, usize, Vec<Key>)> = Vec::new(); // vector tuple: ([id], [weight], [directed], [source node number], [target node number], Vec[keys])

            // 3.1 Switch to graph node (there must be only exactly one graph node (first one found is selected!)):
            if let Some(graph_node) = xdoc.children().find(|n| n.name() == GRAPH_NAME) {
                // 3.1.1 Graph Id has a special meaning: Because it must already be specified in the constructor, its validity must be checked here!
                let graphid = match graph_node.attr("id") {
                    Some(graphid_value) => {
                        // Box the &str to make sure it lives as long as lifetime parameter 'a lives.
                        let value = Box::leak(graphid_value.to_string().into_boxed_str()) as &'a str;
                        Some(value) /* graph id is always unique because application supports only one graph node */},
                    None => Self::errfun(&mut error_string, &format!("Missing graph id attribute!\n")),
                };
                if let None = graphid {
                    // Because of that determination, all other evaluation of graphid is unnecessary
                    return Err(Box::new("Missing graph id!\n".parse().unwrap()));
                }

                // 3.1.2 Determine whether theres a global instruction about edges direction
                // It is not forced to define that globally! So its not necessary to produce a error message if it fails!
                let edgedefault = match graph_node.attr("edgedefault") {
                    Some(edgedefault_value) => match edgedefault_value {
                        "directed" => Some(GraphType::Directed),
                        "undirected" => Some(GraphType::Undirected),
                        _ => None,
                    },
                    None => None,
                };

                // 4. Nodes.
                {
                    // 4.1 Needed temp variables:
                    // Counter to determine how many nodes are created:
                    let mut counter: usize = 0;
                    // HashSet to detect nodes with same id to ensure that no id is used more than once:
                    let mut node_ids: HashSet<&str> = HashSet::new();

                    /* 4.2 Iterates through all node elements. Each node element must have the following attributes/nodes:
                    Attributes:
                    - id (String)
                    Nodes: (optional)
                    - default

                    Each default node must have the following attributes/nodes:
                    Attributes:
                    - key (String)
                    Nodes:
                    - none but a default value must placed inside default node (String)

                    If any attribute is missing, node cannot be generated!
                    */
                    for (i, node) in graph_node
                        .children()
                        .filter(|n| n.name() == NODE_NAME)
                        .enumerate()
                    {
                        // 4.2.1 id
                        let id = match node.attr("id") {
                            Some(id_value) if Self::unused_id(&node_ids, id_value) => {
                                // Valid node id.

                                // Box &str to make sure it does live as long as lifetime parameter 'a lives.
                                let value = Box::leak(id_value.to_string().into_boxed_str()) as &'a str;

                                node_ids.insert(id_value); // update node id set (must not be same lifetime!)
                                Some(value)
                            }
                            Some(id_value) => {
                                // Multiple used id.
                                Self::errfun(
                                    &mut error_string,
                                    &format!("Multiple used node id! (id:{})\n", id_value),
                                )
                            }
                            None => {
                                Self::errfun(&mut error_string, &format!("Missing node id! (#{})\n", i))
                            }
                        };

                        // 4.2.2 Determine whether id is valid and collect all keys to finally have all information to create a tuple in nodes vector:
                        if let Some(id_value) = id {
                            // It is not forced that a node must have keys and default value, so this could also be done here first:
                            // 4.2.2.1 Collects all data elements the node might have.
                            let datas_node = Self::create_data_list(node, id_value);

                            // 4.2.2.2 Check all keys that affect nodes or all elements: (ref keyword is necessary to avoid a move and use a reference binding instead; matches!-macro is useful to make a comparison with enum values)
                            let keys_node = Self::key_fusion(&keys_map, datas_node, |x| {
                                (x == &KeyFor::All || x == &KeyFor::Node)
                            }); // TODO: Gut testen, mögliche Fehlerquelle!

                            // Increase counter:
                            counter += 1;

                            // 4.2.2.3 Here: All information are gathered to create new node:
                            nodes.push((id_value, keys_node, i));
                        } else {
                            return Err(Box::from(format!("Found node errors:\n{}", error_string)));
                        }
                    }

                    // 4.2.3 Additional constraints to check:
                    // - There must be at least two nodes
                    if counter < 2 {
                        return Err(Box::from(format!(
                            "Found node errors:\nGraph must have at least two nodes!\n{}",
                            error_string
                        )));
                    }
                }

                // 5. Edges:
                {
                    // 5.1 Needed temp variables:
                    // Counter to determine how many nodes are created:
                    let mut counter: usize = 0;
                    // HashSet to detect edges with same id to ensure that no id is used more than once:
                    let mut edge_ids: HashSet<&str> = HashSet::new();

                    // 5.2 Needed closures:
                    // Searches in all known nodes for the one with specified id (At this point: Its ensured that all node id's are unique)
                    let node_ref = |src: &str| -> Option<usize> {
                        nodes.iter().find_map(|(id, _, no)| {
                            if *id == src {
                                Some(*no)
                            } else {
                                None
                            }
                        })
                    };

                    /* 5.3 Iterates through all edge elements. Each edge element must have the following attributes/nodes:
                    Attributes:
                    - id (String)
                    - directed (GraphType)
                    - weight (u32)
                    - source (String)
                    - target (String)
                    Nodes: (optional)
                    - default

                    Each default edge must have the following attributes/nodes:
                    Attributes:
                    - key (String)
                    Nodes:
                    - none but a default value must placed inside default node (String)

                    If any attribute is missing, edge cannot be generated!
                    */
                    for (i, edge) in graph_node
                        .children()
                        .filter(|n| n.name() == EDGE_NAME)
                        .enumerate()
                    {
                        // 5.3.1 id
                        let id = match edge.attr("id") {
                            Some(id_value) if Self::unused_id(&edge_ids, id_value) => {
                                // Valid edge id.
                                // Box &str to make sure it does live as long as lifetime parameter 'a lives.
                                let value = Box::leak(id_value.to_string().into_boxed_str()) as &'a str;

                                edge_ids.insert(id_value); // update edge set (must not be same lifetime!)
                                Some(value)}
                            Some(id_value) => {
                                // Id exists but is multiple used!
                                Self::errfun(
                                &mut error_string,
                                &format!("Multiple used edge id! (id:{})\n", id_value),
                            )},
                            None => {
                                // No id attribute was found!
                                Self::errfun(
                                &mut error_string,
                                &format!("Missing edge id attribute! (#{})\n", i),
                            )},
                        };

                        // 5.3.2 directed:
                        // Uses the setting found in the edge attribute. If the attribute is not found global value is applied.
                        // In any other cases: E.g. attribute not found; no global value defined: Throw exception!
                        let directed = match edge.attr("directed") {
                            Some(directed_value) => match directed_value {
                                "directed" => Some(GraphType::Directed),
                                "undirected" => Some(GraphType::Undirected),
                                _ => {
                                    // Invalid value in attribute!
                                    Self::errfun(
                                        &mut error_string,
                                        &format!(
                                            "Invalid value for edge directed attribute! (#{})\n",
                                            i
                                        ),
                                    )
                                }
                            },
                            None => {
                                match &edgedefault /* An immutable reference is used here otherwise a move is assumed! */ {
                                    Some(default) => Some(default.clone()),
                                    None => {
                                        // Attribute was not found!
                                        Self::errfun(&mut error_string, &format!("Neither default edge direction nor directed attribute found! (#{})\n", i))
                                    },
                                }
                            }
                        };

                        // 5.3.3 weight:
                        let weight = match edge.attr("weight") {
                            Some(weight_value) => {
                                let weight_parsed = weight_value.parse::<u32>(); // try to parse value into u32... (TURBOFISH!)
                                // ... evaluate result:
                                match weight_parsed {
                                    Ok(val) => Some(val),
                                    Err(_) => {
                                        // Error occured during parsing so probably invalid value:
                                        Self::errfun(
                                        &mut error_string,
                                        &format!("Invalid value for edge weight! (#{})\n", i),
                                    )
                                    },
                                }
                            }
                            None => {
                                // No attribute was found:
                                Self::errfun(
                                &mut error_string,
                                &format!("Missing edge weight attribute! (#{})\n", i),
                            )
                            },
                        };

                        // 5.3.4 source:
                        let source = match edge.attr("source") {
                            Some(source_value) => {
                                if let Some(reference) = node_ref(source_value) {
                                    Some(reference) // valid reference
                                } else {
                                    // Undefined reference: Value in attribute could not mapped to a real node id!
                                    Self::errfun(
                                        &mut error_string,
                                        &format!("Undefined source reference! (#{})\n", i),
                                    )
                                }
                            }
                            None => {
                                // No attribute found!
                                Self::errfun(
                                    &mut error_string,
                                    &format!("Missing edge source attribute! (#{})\n", i),
                                )
                            }
                        };

                        // 5.3.5 destination:
                        let destination = match edge.attr("target") {
                            Some(target_value) => {
                                if let Some(reference) = node_ref(target_value) {
                                    Some(reference) // valid reference
                                } else {
                                    // Undefined reference: Value in attribute could not mapped to a real node id!
                                    Self::errfun(
                                        &mut error_string,
                                        &format!("Undefined target reference! (#{})\n", i),
                                    )
                                }
                            }
                            None => {
                                // No attribute found!
                                Self::errfun(
                                    &mut error_string,
                                    &format!("Missing edge target attribute! (#{})\n", i),
                                )
                            }
                        };

                        // 5.3.6 Perform validity check for all necessary variables to finally create edge tuple in vector:
                        if id.is_some() && directed.is_some() && weight.is_some() && source.is_some() && destination.is_some() {
                            // It is not forced that a node must have keys and default value, so this could also be done here first:
                            // 5.3.6.1 Collects all data elements the edge might have and iterates through all data elements of the edge.
                            let datas_edge = Self::create_data_list(edge, id.unwrap());

                            // Increment edge counter:
                            counter += 1;

                            // 5.3.6.2 Check all keys that affect edges or all elements: (ref keyword is necessary to avoid a move and use a reference binding instead; matches!-macro is useful to make a comparison with enum values)
                            let keys_edge = Self::key_fusion(&keys_map, datas_edge, |x| {
                                (x == &KeyFor::All) | (x == &KeyFor::Edge)
                            });

                            // 5.3.6.3 Here: All information is available to create new edge:
                            edges.push((
                                id.unwrap(),
                                weight.unwrap(),
                                directed.unwrap(),
                                source.unwrap(),
                                destination.unwrap(),
                                keys_edge,
                            ));
                        } else {
                            return Err(Box::from(format!("Found edge errors:\n{}", error_string)));
                        }
                    }

                    // 5.3.7 Additional constraints to check:
                    // - There must be at least one edge (at this point all edges are valid and therefore their node references too!)
                    if counter == 0 {
                        return Err(Box::from(format!(
                            "Found node errors:\nGraph must have at least one edge!\n{}",
                            error_string
                        )));
                    }
                }

                // 6. Graph object:
                // 6.1 Select keys that affect graph object: (for better readability create_data_list was passed here as parameter (and to avoid name conflicts with following variables)
                let keys_graph =
                    Self::key_fusion(&keys_map, Self::create_data_list(&graph_node, GRAPH_NAME), |x| {
                        (x == &KeyFor::All) || (x == &KeyFor::Graph)
                    });


                // 6.2 Create SmartPointer objects for nodes and edges.
                let rc_nodes: Vec<Rc<Node>> = nodes
                    .into_iter()
                    .map(|n| {
                        let (id, keys, no) = n;
                        Rc::new(Node::new(id, keys, no))
                    })
                    .collect();

                let rc_edges: Vec<Rc<Edge>> = edges
                    .into_iter()
                    .map(|n| {
                        let (id, weight, etype, src_no, dst_no, keys) = n;
                        Rc::new(Edge::new(id, weight, etype, Rc::clone(&rc_nodes[src_no]), Rc::clone(&rc_nodes[dst_no]), keys))
                    })
                    .collect();


                // 6.3 Call graph constructor and return object:


                return Ok(Graph::new(
                    graphid.unwrap(),
                    rc_nodes,
                    rc_edges,
                    keys_graph,
                ));
            }
            else {
                // No "graph" node was found!
                return  Err(Box::from("No graph element was found!\n".to_string()));
            }
        } else {
            // No root element was found!
            return Err(Box::from("No root graphml-node was found!\n".to_string()));
        }
    }
}