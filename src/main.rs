mod Graph;
mod Dijkstra;
mod GraphML;
mod GraphOutput;
mod GraphPositioning;

// Standard library.
use std::io::Write; // used for command line output
use std::env; // environment - to get current path
use std::fs; // file system manipulation

use minidom::Element; // xml parser

// Own objects.
use crate::Graph::node::Node;
use crate::Graph::edge::Edge;
use crate::Graph::graph_type::graph_enum::GraphType;
use crate::Graph::IgraphObject;

const NS: &str = "http://graphml.graphdrawing.org/xmlns";

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

    // Get the graph element (no root node!).
    let graph: Option<&Element> = root.get_child("graph", NS);
    if graph.is_none() {
        errors.push("No graph element".to_string());
        return; // at this point it makes no sense to go further.
    }

    let graphId: Option<&str> = graph.unwrap().attr("id");
    if graphId.is_none() {
        errors.push(format!("Missing graph id"));
        
    }
    
    let graphId: String = match graph.unwrap().attr("id") {
        Some(id) => id.to_string(),
        None => {
            errors.push(format!("Missing graph id"));
            // if this is the only error then continue. Not having an ID for the graph is not that critical. But a default value is used instead.
            "unknown".to_string() // because of missing ; this is treated correctly as return value !
        }
    };

    // Iterate over node elements.
    for (index, node) in graph.unwrap().children().filter(|e: &&Element | e.name() == "node").enumerate() {
        let id: Option<&str> = node.attr("id");

        if id.is_none() {
            errors.push(format!("Missing node id: {}", index));
        } else {
            // add element in node list.
            nodes.push(Node::new(String::from(id.unwrap()), Vec::new(), index as u32));
        }
    }

    // Iterate over edge elements.
    for (index, edge) in graph.unwrap().children().filter(|e: &&Element | e.name() == "edge").enumerate() {
        let mut invalid = false; // set to true if at least one invalid parameter was found/read.

        // id of the edge
        let id: Option<&str> = edge.attr("id");
        if id.is_none() {
            errors.push(format!("Missing edge id: {}", index));
            invalid = true;
        };

        // type of the edge (directed / undirected)
        let kind: Option<&str> = edge.attr("directed");
        let status: Result<GraphType, String> = match kind {
            Some(k) => k.parse(), // try to parse the string into GraphType
            None => Err(format!("Invalid edge kind: {}", index)),
        };
        if status.is_err() {
            errors.push(format!("Invalid edge directed: {}", index));
            invalid = true;
        }

        // source node (already parsed into existing node id)
        let source: Option<&Node>= edge
            .attr("source")
            .and_then(|source_id| nodes.iter().find(|n: &&Node |n.get_id() == source_id));
        if source.is_none() {
            errors.push(format!("Missing/invalid source id: {}", index));
            invalid = true;
        }

        // target node (already parsed into existing node id)
        let target: Option<&Node> = edge
            .attr("target")
            .and_then(|target_id| nodes.iter().find(|n: &&Node |n.get_id() == target_id));
        if target.is_none() {
            errors.push(format!("Missing/invalid target id: {}", index));
            invalid = true;
        }

        let weight: Option<u32> = edge
            .attr("weight")
            .and_then(|s| s.parse().ok());
        if weight.is_none() {
            errors.push(format!("Missing/invalid weight id: {}", index));
            invalid = true;
        }

        if invalid {
            continue;
        }

        let id: String = id.unwrap().to_string();
        let source: &Node = source.unwrap();
        let target: &Node = target.unwrap();
        let weight: u32 = weight.unwrap();
        let status: GraphType = status.unwrap();

        edges.push(Edge::new(id, weight, status, source, target, Vec::new()));
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
