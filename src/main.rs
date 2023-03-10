mod Graph;
mod Dijkstra;
mod Constants;
mod GNUPlotPrinter;
mod GraphOptimization;

// Own objects.
use Graph::node::Node;
use Graph::edge::Edge;

//use Dijkstra::Dijkstra as Dijkstra;
use GNUPlotPrinter::GNUplotPrinter as GNUplotPrinter;

// Standard library.
use std::io::Write;
use crate::Graph::graph_type::graph_enum::GraphType::{Undirected, Directed};

fn main() {
    let mut nodes: Vec<Node> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();

    nodes.push(Node::new("A".to_string(), Vec::new(), 0));
    nodes.push(Node::new("B".to_string(), Vec::new(), 1));
    nodes.push(Node::new("C".to_string(), Vec::new(), 2));
    nodes.push(Node::new("D".to_string(), Vec::new(), 3));
    nodes.push(Node::new("E".to_string(), Vec::new(), 4));
    nodes.push(Node::new("F".to_string(), Vec::new(), 5));
    nodes.push(Node::new("G".to_string(), Vec::new(), 6));
    nodes.push(Node::new("H".to_string(), Vec::new(), 7));
    nodes.push(Node::new("I".to_string(), Vec::new(), 8));
    nodes.push(Node::new("J".to_string(), Vec::new(), 9));
    nodes.push(Node::new("K".to_string(), Vec::new(), 10));


    edges.push(Edge::new("e0".to_string(), 6, Undirected, &nodes[0], &nodes[6], Vec::new()));
    edges.push(Edge::new("e1".to_string(), 1, Undirected, &nodes[7], &nodes[6], Vec::new()));
    edges.push(Edge::new("e2".to_string(), 3, Undirected, &nodes[8], &nodes[0], Vec::new()));
    edges.push(Edge::new("e3".to_string(), 5, Undirected, &nodes[9], &nodes[0], Vec::new()));
    edges.push(Edge::new("e4".to_string(), 7, Undirected, &nodes[1], &nodes[7], Vec::new()));
    edges.push(Edge::new("e5".to_string(), 2, Undirected, &nodes[1], &nodes[9], Vec::new()));
    edges.push(Edge::new("e6".to_string(), 4, Undirected, &nodes[1], &nodes[2], Vec::new()));
    edges.push(Edge::new("e7".to_string(), 3, Undirected, &nodes[2], &nodes[3], Vec::new()));
    edges.push(Edge::new("e8".to_string(), 2, Undirected, &nodes[3], &nodes[10], Vec::new()));
    edges.push(Edge::new("e9".to_string(), 2, Undirected, &nodes[3], &nodes[4], Vec::new()));
    edges.push(Edge::new("e10".to_string(), 8, Undirected, &nodes[10], &nodes[5], Vec::new()));
    edges.push(Edge::new("e11".to_string(), 6, Undirected, &nodes[4], &nodes[7], Vec::new()));
    edges.push(Edge::new("e12".to_string(), 4, Undirected, &nodes[5], &nodes[6], Vec::new()));
    edges.push(Edge::new("e13".to_string(), 3, Undirected, &nodes[3], &nodes[9], Vec::new()));


    let graph = Graph::Graph::new("graph1".to_string(), nodes.clone(), edges.clone(), Vec::new());

    let result = Dijkstra::Dijkstra::run(&graph, &nodes[0]);

    let opt = GraphOptimization::GraphOptimization::run(&graph, &nodes[0]);

    GNUplotPrinter::write2File("Graph.dat".to_string(), &graph, &opt, Option::Some(&result));


// (Everything works in this comment)
    println!("{}", Constants::INTRO);
    print!("   GraphML-filepath: ");

    // force stream to flush
    std::io::stdout().flush().unwrap();
 
    let mut filepath = String::new();

    std::io::stdin()
        .read_line(&mut filepath)
        .expect("Error while reading...");

    



    //println!("{}", filepath);
}
