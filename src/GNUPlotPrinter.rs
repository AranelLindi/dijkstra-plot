// Own objects.
use crate::Dijkstra::DijkstraHeapEle;
use crate::Graph::{Graph, IgraphObject};
use crate::GraphOptimization::NodePos;

// Standard library.
use std::fs::File;
use std::io::Write;

struct NodePlot {
    no: u32,
    x: f32,
    y: f32,
    id: String,
    marked: bool,
}

impl NodePlot {
    fn new(no: u32, x: f32, y: f32, id: String, marked: bool) -> Self {
        NodePlot {
            no,
            x,
            y,
            id,
            marked,
        }
    }
}

struct EdgePlot {
    from: u32,
    to: u32,
    weight: u32,
    marked: bool,
}

impl EdgePlot {
    fn new(from: u32, to: u32, weight: u32, marked: bool) -> Self {
        EdgePlot {
            from,
            to,
            weight,
            marked,
        }
    }
}

pub struct GNUplotPrinter<'a> {
    marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> GNUplotPrinter<'a> {
    pub fn write2File(file_name: String, graph: &'a Graph, positions: &'a Vec<NodePos>, dijkstra: Option<&Vec<DijkstraHeapEle>>) {
        let node_len = graph.node_len as usize;
        let edge_len = graph.edge_len as usize;

        // Create file:
        let mut file = File::create(file_name).unwrap();

/*        if Err(err) = cfile {
            // Serious error occurred, it makes no sense to continue
            return;
        }*/

        // Sort nodes ascending by node number.
        let mut positions_sorted = positions.to_vec(); // .to_vec() creates a deep copy of the vector while .clone() would return reference to vector!
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
                if let Some(paths) = dijkstra {
                    if let Some(_) = paths.iter().find(|&x| x.owner == ref_node) {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };

            nodes_plot.push(NodePlot::new(
                ref_node.no(),
                x,
                y,
                ref_node.get_id().to_string(),
                marked,
            ));
        }

        // second for edges:
        for _ in 0..edge_len {}

        for edge in graph.edges.iter() {
            let from = edge.source().no();
            let to = edge.dest().no();
            let weight = edge.weight();

            let marked = {
                if let Some(paths) = dijkstra {
                    if let Some(_) = paths.iter().find(|&x| {
                        if let Some(prev) = x.prev {
                            if (x.owner == edge.source() && prev == edge.dest()) || (x.owner == edge.dest() && prev == edge.source()) {
                                return true;
                            } else { return false; }
                        } else { return false; } }) { /* return value if elements in paths that match closure were find */ true }
                    else { /* return value if no elements in paths that match closure are not found */ false }
                } else { /* return value if no dijkstra path was given as parameter */ false }
            };

            edges_plot.push(EdgePlot::new(
                from,
                to,
                weight,
                marked,
            ));
        }

        // After all necessary information about nodes and edges has been created, start writing them to file:
/*        if let Ok(file) = cfile {
            // Write nodes:
            for i in 0..=node_len - 1 {
                let ref_node = &graph.nodes[i];
                let ref_pos = &positions[graph.nodes[i].no() as usize];
                let (x, y) = ref_pos.pos;
            }
        } else {
            // error
        };*/

        // Write nodes first:
        for node in nodes_plot.iter() {
            writeln!(file, "{} {} {} {} {}", node.no, node.x, node.y, node.id, if node.marked { "1" } else { "0" }).unwrap();
        }

        // empty line between nodes and edges:
        writeln!(file, "").unwrap();

        // After that, edges:
        for edge in edges_plot.iter() {
            let src_node_ref = &nodes_plot[edge.from as usize];
            let dst_node_ref = &nodes_plot[edge.to as usize];

            let (src_x, src_y) = (src_node_ref.x, src_node_ref.y);
            let (dst_x, dst_y) = (dst_node_ref.x, dst_node_ref.y);

            writeln!(file, "{} {} {} {} {} {}", src_x, src_y, dst_x, dst_y, edge.weight, if edge.marked { "1" } else { "0" }).unwrap();
        }
    }
}
