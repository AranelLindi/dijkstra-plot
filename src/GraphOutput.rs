/*
    file created by: Stefan Lindörfer, 2023
      A given graph including shortest-path information
      and coordinates of the graph objects are written
      into a text file so python script could read that.
 */

// Own objects.
use crate::Dijkstra::DijkstraHeapEle;
use crate::Graph::{Graph, IgraphObject};
use crate::GraphPositioning::NodePos;

// Standard library.
use std::fs::File;
use std::io::Write;
use std::process::exit;

// Represents graphical form of a node.
struct NodePlot {
    no: usize,
    x: f32,
    y: f32,
    id: String,
    marked: bool,
}

impl NodePlot {
    // Constructor (associative function)
    fn new(no: usize, x: f32, y: f32, id: String, marked: bool) -> Self {
        NodePlot {
            no,
            x,
            y,
            id,
            marked,
        }
    }
}

// Represents graphical form of an edge.
struct EdgePlot {
    from: usize,
    to: usize,
    weight: u32,
    marked: bool,
}

impl EdgePlot {
    // Constructor (associative function)
    fn new(from: usize, to: usize, weight: u32, marked: bool) -> Self {
        EdgePlot {
            from,
            to,
            weight,
            marked,
        }
    }
}

pub struct GraphOutput<'a> {
    marker: std::marker::PhantomData<&'a ()>, /* marker is necessary here to be allowed to use lifetime parameters! */
}

impl<'a> GraphOutput<'a> {
    // Writes a graph and result of dijkstra algorithm and information about the positioning of the graph into a text file:
    pub fn write2File(file_name: String, graph: &'a Graph, position_information: &'a Vec<NodePos>, dijkstra_information: Option<&Vec<DijkstraHeapEle>>) {
        let node_len = graph.node_len as usize;
        let edge_len = graph.edge_len as usize;

        // Create file:
        let mut rfile = File::create(file_name.clone()); // has to be mutable?
        let mut file: File;// = rfile.ok().unwrap();

        // Check if file could be created. If not exit application with error code.
        if let Err(_) = rfile {
            println!("Unable to create output file {}", file_name.clone());
            exit(1);
        }
        else {
            file = rfile.ok().unwrap();
        }

        // Sort nodes ascending by node number.
        let mut positions_sorted = position_information.to_vec(); // .to_vec() creates a deep copy of the vector while .clone() would return reference to vector!
        positions_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut nodes_plot: Vec<NodePlot> = Vec::new();
        let mut edges_plot: Vec<EdgePlot> = Vec::new();

        // Create objects for nodes first:
        for i in 0..node_len {
            let ref_node = &graph.nodes[i];
            let ref_pos = &positions_sorted[i]; // positions must be sorted by node number !

            let (x, y) = ref_pos.pos;

            // Find out if current node is part of dijkstra path:
            let marked: bool = {
                if let Some(paths) = dijkstra_information {
                    if let Some(_) = paths.iter().find(|&x| x.owner == ref_node) {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };

            // Creates new object with coordinates and co. and pushes it into vector.
            nodes_plot.push(NodePlot::new(
                ref_node.no(),
                x,
                y,
                ref_node.get_id().to_string(),
                marked,
            ));
        }

        // Second for edges:
        for edge in graph.edges.iter() {
            // Current edges data:
            let from = edge.source().no();
            let to = edge.dest().no();
            let weight = edge.weight();

            // Find out if current edge is part of a shortest path. If so, mark it!
            // Therefore check first if edge is connected with two nodes that are part of shortest path vector (so it must be excluded that two nodes have more than one connecting edge!)
            // Code works but there is a shorter and readable (and more elegant) code below it:
/*            let marked = {
                if let Some(paths) = dijkstra_information {
                    if let Some(_) = paths.iter().find(|&x: &DijkstraHeapEle| {
                        if let Some(prev) = x.prev /* It is not mandatory that each node must have predecessor (prev)! */ {
                            // Check for each DijkstraHeapEle if it is connected to the nodes the current edge is connected to also. Because it is excluded that there is more than one connection between to nodes it MUST be the current edge! Return true then otherwise false
                            return (x.owner == edge.source() && prev == edge.dest()) || (x.owner == edge.dest() && prev == edge.source()) // {return true} else { return false }
                        } else { return false; } }) { /* return value if elements in paths that match closure were find */ true }
                    else { /* return value if no elements in paths that match closure are not found */ false }
                } else { /* return value if no dijkstra path was given as parameter */ false }
            };
*/
            // TODO: Test if this code works as well! -> Seems so!
            let marked = dijkstra_information.map_or(false, |paths| {
                paths.iter().any(|path| {
                    if let Some(prev) = path.prev {
                        // Remember: Edge could also be reversed! So check in both directions as well!
                        match (path.owner == edge.source(), prev == edge.dest(), path.owner == edge.dest(), prev == edge.source()) {
                            (true, true, _, _) | (_, _, true, true) => true, // Check for each DijkstraHeapEle if it is connected to the nodes the current edge is connected to also. Because it is excluded that there is more than one connection between to nodes it MUST be the current edge!
                            _ => false,
                        }
                    } else { false }
                })
            });

            // Creates new object with required information and push it into vector.
            edges_plot.push(EdgePlot::new(
                from,
                to,
                weight,
                marked,
            ));
        }

        // Write nodes first:
        for node in nodes_plot.iter() {
            writeln!(file, "{} {} {} {} {}", node.no, node.x, node.y, node.id, if node.marked { "1" } else { "0" }).unwrap();
        }

        // Empty line between nodes and edges (very important for python script!)
        writeln!(file, "").unwrap();

        // Then it is turn of edges:
        for edge in edges_plot.iter() {
            // Since EdgePlot has no fields for coordinates a simple conversion is here made through by inserting coordinates of start and end nodes
            // References:
            let src_node_ref = &nodes_plot[edge.from as usize];
            let dst_node_ref = &nodes_plot[edge.to as usize];

            // Coordinates of referenced nodes:
            let (src_x, src_y) = (src_node_ref.x, src_node_ref.y); // packing
            let (dst_x, dst_y) = (dst_node_ref.x, dst_node_ref.y); // packing

            // Write information into file:
            writeln!(file, "{} {} {} {} {} {}", src_x, src_y, dst_x, dst_y, edge.weight, if edge.marked { "1" } else { "0" }).unwrap();
        }
    }
}