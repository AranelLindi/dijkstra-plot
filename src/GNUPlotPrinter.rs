use std::fs::File;

struct GNUplotPrinter {}

impl GNUplotPrinter {
    pub fn write2File(graph: graph::Graph, file: String) {
        let mut file = File::create("graph.dat");
    }
}