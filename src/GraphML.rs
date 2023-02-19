extern crate minidom;

use minidom::Element;

struct GraphML {}

impl GraphML {
    pub fn createGraph(graphml: String) {


        let root: Element = graphml.parse().unwrap();

        if root.name().to_upper() != "GraphML".to_upper() {
            
        }

        for child in root.children() {

        }
    }
}