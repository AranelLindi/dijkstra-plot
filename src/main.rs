mod Graph;
mod Dijkstra;
mod GraphML;
mod GraphOutput;
mod GraphPositioning;
mod KeyCollection;

// Standard library.
use std::io::Write; // used for command line output
use std::env; // environment - to get current path
use std::fs; // file system manipulation

use minidom::Element; // xml parser

// Own objects.
use crate::Graph::node::Node;
use crate::Graph::edge::Edge;
use crate::Graph::graph_type::graph_enum::GraphType;
use crate::Graph::{IgraphObject, Key};
//use crate::GraphML::key_for::KeyFor::Node;
use crate::KeyCollection::{collect_keys_for, AllScope, EdgeScope, NodeScope};

const NS: &str = "http://graphml.graphdrawing.org/xmlns";

// Macro to easily collect error messages in a vector.
/* Explanation:
 * :expr - any expression (e.g. edge.attr("id")
 * :literal - string or number (e.g. "id")
 * :ident - identifier (not a string or an expression but a variable like bool)
 */
macro_rules! get_attr {
    ($attr:expr, $errtype:literal, $name:literal, $index:expr, $description:literal, $errors:expr, $invalid:ident) => {{
        if $attr.is_none() {
            $errors.push(format!(
                "{}: {} ({}): {}",
                $errtype, $name, $index, $description
            ));
            $invalid = true;
        }
        "" // dummy string return value so it can be used in expression context
    }};
}


// Helper functions
fn findNode<'a>(nodes: &'a [Node], id: &str) -> Option<&'a Node> {
    nodes.iter().find(|node| node.get_id() == id)
}


// Parsing functions
fn parseKey() -> Option<Key> {
    None // TODO: Has to be implemented yet! See KeyCollection.rs !
    /*
        use minidom::Element;
    
    // Angenommen, das hier ist dein Wurzel-Element:
    let root: Element = parse_graphml()?;
    
    // Sammle alle <key>-Elemente
    let keys: Vec<Element> = root
        .children()
        .filter(|e| e.name() == "key")
        .cloned()
        .collect();
    
    // Jetzt je nach Scope aufrufen
    let node_keys = collect_keys_for::<NodeScope>(&keys);
    let edge_keys = collect_keys_for::<EdgeScope>(&keys);
    let graph_keys = collect_keys_for::<GraphScope>(&keys);

     */
}

fn assign_key_for_node(graph_keys: &[Key], node_keys: &[Key]) -> Vec<Key> {
    let mut keys = Vec::new();
    keys.extend_from_slice(graph_keys);
    keys.extend_from_slice(node_keys);
    keys
}

fn assign_key_for_edge(graph_keys: &[Key], edge_keys: &[Key]) -> Vec<Key> {
    let mut keys = Vec::new();
    keys.extend_from_slice(edge_keys);
    keys.extend_from_slice(graph_keys);
    keys
}

fn assign_key_for_graph(graph_keys: &[Key]) -> Vec<Key> {
    let mut keys = Vec::new();
    keys.extend_from_slice(graph_keys);
    keys
}

fn parseNode(node: &Element, index: usize, errors: &mut Vec<String>, graph_keys: &[Key], node_keys: &[Key]) -> Option<Node> {
    // Read and convert attribute once
    let id_raw: Option<&str> = node.attr("id");
    
    if id_raw.is_none() {
        errors.push(format!("Missing or invalid id for node at index {}", index));
        None
    } else {
        Some(Node::new(id_raw.unwrap().to_string(), assign_key_for_node(graph_keys, node_keys), index as u32))
    }

    // TODO: Here the get_attr! Macro must be implemented to make visible if an information is missing or invalid !
}

fn parseEdge<'a>(edge: &Element, nodes: &'a [Node], index: usize, errors: &mut Vec<String>, graph_keys: &[Key], edge_keys: &[Key]) -> Option<Edge<'a>> {
    // TODO: parseEdge does not yet support the read-in of the default nodes for keys within the edge nodes.

    // Read and convert attributes once
    let id_raw = edge.attr("id");
    let kind_raw = edge.attr("directed");
    let source_raw = edge.attr("source");
    let target_raw = edge.attr("target");
    let weight_raw = edge.attr("weight");

    // Extract parsed values
    let kind = kind_raw.and_then(|k| k.parse::<GraphType>().ok());
    let source = source_raw.and_then(|k| findNode(nodes, k));
    let target = target_raw.and_then(|k| findNode(nodes, k));
    let weight = weight_raw.and_then(|w| w.parse::<u32>().ok());

    let mut has_error = false;

    // Use id for context, or fallback to index if missing
    let id_for_error = id_raw.unwrap_or("<missing>");

    if id_raw.is_none() {
        errors.push(format!("Missing id for edge at index {}", index));
        has_error = true;
    }
    if kind.is_none() {
        errors.push(format!("Invalid or missing 'directed' attribute in edge {}", id_for_error));
        has_error = true;
    }
    if source.is_none() {
        errors.push(format!("Invalid or missing 'source' node in edge {}", id_for_error));
        has_error = true;
    }
    if target.is_none() {
        errors.push(format!("Invalid or missing 'target' node in edge {}", id_for_error));
        has_error = true;
    }
    if weight.is_none() {
        errors.push(format!("Invalid or missing 'weight' value in edge {}", id_for_error));
        has_error = true;
    }

    if has_error {
        None
    } else {
        Some(Edge::new(
            id_raw.unwrap().to_string(),
            weight.unwrap(),
            kind.unwrap(),
            source.unwrap(),
            target.unwrap(),
            assign_key_for_edge(graph_keys, edge_keys),
        ))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // All possible runtime parameters.
    let mut input: Option<&str> = None;
    let mut output: Option<&str> = None;
    let mut start:Option<&str> = None;
    let mut dest:Option<&str> = None;

    // Read in passed parameters.
    for arg in args.iter() {
        // strip_prefix removes the given part at the beginning of a string slice.
        if let Some(val) = arg.strip_prefix("-input=") {
            input = Some(val); // GraphMl input path
        } else if let Some(val) = arg.strip_prefix("-output=") {
            output = Some(val); // Output file
        } else if let Some(val) = arg.strip_prefix("-start=") {
            start = Some(val); // Start node
        } else if let Some(val) = arg.strip_prefix("-dest=") {
            dest = Some(val); // Destination node
        }
    }

    //let input = input.unwrap_or("Graph.xml");
    //let output = output.unwrap_or("Graph.dat");
    //let start = start.unwrap_or("A");
    //let dest = dest.unwrap_or(""); // no dest graph: dijkstra is performed for complete graph
    // TODO: std::process::exit(1) bedeutet, dass das Programm mit einem Fehler beendet wurde. Dieses also bei ungültigen Parametern etc. zurückgeben. Das Bash Skript fängt das auf und kann dann entscheiden ob weiter gemacht werden soll

    // Give some fundamental information just to exclude common mistakes.
    println!("Current dir: {}", std::env::current_dir().unwrap().display());
    println!("Trying to read: '{}'", input.unwrap());

    // Read in the file ...
    let xml_str = fs::read_to_string(input.unwrap()).expect("Something went wrong reading the file");
    // ... and parse it as XML.
    let root: Element = xml_str.parse().expect("Failed to parse XML");

    // Containers to store the graph elements.
    let mut nodes: Vec<Node> = Vec::new(); // stores all nodes
    let mut edges: Vec<Edge> = Vec::new(); // stores all edges with references to nodes

    let mut errors: Vec<String> = Vec::new(); // contains all error messages that occur

    let mut invalid = false; // indicates if a parsing error occurred
    
    // Get the graph element (no root node!).
    let graph: Option<&Element> = root.get_child("graph", NS);
    
    if graph.is_none() {
        //errors.push("No graph element".to_string());
        get_attr!(graph, "Err", "graph", 0, "no graph element", errors, invalid);
        return; // at this point it makes no sense to go further. // TODO: Think about a solution to make this error also visible to the user even if the program stops here !
    }

    let graph = graph.unwrap(); // At this point a graph element exists !
    let attr = graph.attr("id");
    get_attr!(attr, "Warn", "graphId", 0, "Missing graph id", errors, invalid);
    let graphId = attr.unwrap_or("unknown").to_string();


    // Iterate over node elements.
    /*for (index, node) in graph.children().filter(|e: &&Element | e.name().to_lowercase() == "node").enumerate() {
        match parseNode(node, index, &mut errors) {
            Some(node) => {
                nodes.push(node);
            }
            None => {
                continue
            }
        }
    }*/

    let key_elements: Vec<Element> = root.children()
        .filter(|e| e.name() == "key" && e.ns() == NS)
        .cloned()
        .collect();
    let keysForEdges = collect_keys_for::<EdgeScope>(&key_elements);
    let keysForNodes = collect_keys_for::<NodeScope>(&key_elements);
    let keysForAll = collect_keys_for::<AllScope>(&key_elements);

    for (_, node) in graph.children()
        .filter(|e: &&Element | e.name().eq_ignore_ascii_case("node")) // 1. filters only elements with name equal to "node" and returns bool [(Auto-Dereferencing! e.name() means: (**e).names())]
        .enumerate() // 2. enumerates all filtered elements and provides (index: usize, node: &Element)
        .filter_map(|(index , node)| {
            parseNode(node, index, &mut errors, &keysForAll, &keysForNodes) // 4. after parseNode() and map() are executed on every piece filter_map removes all Nones and returns the Some values, resulting the (_, node: Node) iterator, used in the for-loop
                .map(|n| (index, n)) // 3. map takes the result from parseNode (Option<Node>) and turns it into Option<(index, Node)> receives Option<Node> from filter_map and converts it to Option<(index, Node)>, needed for for-loop structure
        }) // map returns Option<Node> and filter_map returns an iterator consisting of Node thats why node in the for-loop is of type Node and not &Element !
    {
        nodes.push(node); // 5. node is of type Node here (not Option!) because filter_map unwraps the Some(...)
    }

    //for (index, edge) in graph.unwrap().children().filter()

    for (_, edge) in graph.children()
        .filter(|e: &&Element | e.name().eq_ignore_ascii_case("edge"))
        .enumerate()
        .filter_map(|(index, edge)| parseEdge(edge, &nodes, index, &mut errors, &keysForAll, &keysForEdges).map(|e| (index, e))) {
        edges.push(edge)
    }

    
    // Print out all gathered error messages:
    println!("Errors: {}", errors.join("\n")); // join() connects all elements in the vector to one single string seperated through new lines

    let start: Option<&Node> = nodes.iter().find(|n: &&Node |n.get_id() == start.unwrap());
    let dest: Option<&Node> = nodes.iter().find(|n: &&Node |n.get_id() == dest.unwrap());

    if start.is_none() {
        errors.push(format!("Missing start node"));
        return;
    }


    let graph = Graph::Graph::new(String::from(graphId), nodes.clone(), edges.clone(), Vec::new());

    let result = Dijkstra::Dijkstra::run(&graph, start.unwrap());

    let opt = GraphPositioning::GraphOptimization::run(&graph, start.unwrap());

    GraphOutput::GraphOutput::write2File(output.unwrap().to_string(), &graph, &opt, Some(&result));

    println!("Graph success!");

}
