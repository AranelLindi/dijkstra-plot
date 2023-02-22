mod Node;
mod Edge;
mod IGraphObject;
mod Key;
mod KeyType;


struct Graph {
    id: str,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    keys: Vec<Key>
}

impl GraphObject for Graph {
    fn new(&mut self, id: Box<str>, nodes: Vec<Node>, edges: Vec<Edge>, keys: Vec<Key>) {
        self.id = *id; // probably dereference to use the object in the box
        self.nodes = nodes;
        self.edges = edges;
        self.keys = keys;
    }

    fn getAdjacencyMatrix(&self) -> [[u8; N]; N] { // TODO could make problems here with unknown array size...
        const N : u32 = self.nodes.len() as u32;

        // creates 2D array respectively N x N matrix within N := len(self.nodes)
        let mut matrix = [[0u8; N]; N];

        // enumerate returns tuple with (position, current object)
        for (_, e) in self.edges.iter().enumerate() {
            matrix[e.source.no][e.dest.no] = 1u8;
        }

        //let return_matrx = matrix;

        return matrix;
    }

    fn addKey(obj: &mut GraphObject, key: &Key) {
        // find out if key already exists
        if let Some(index) = obj.keys.iter().position(| &x | x.getID() == key.id) {
            // element exists so update value
            obj.keys[index].value = key.value;
        }
        else {
            // element is not yet part of vector, so add it
            obj.keys.push(key);
        }
    }

    fn deleteKey(obj: &GraphObject, id: &str) {
        if let Some(index) = *obj.keys.iter().position(| &x | x.id == *id) { // TODO: Is *id correct? Compiler doesn't say anything.. even with just "id" - CHECK!
            // remove object
            obj.keys.remove(index);
        } // (else: element not found in vector so nothing needs to be done)
    }

    fn getPosKeyById(obj: &GraphObject, id: &str) -> i32 {
        if let Some(index) = *obj.keys.iter().position(| &x | x.id == *id) {
            index
        }
        else {
            -1 // Könnte man vielleicht auch anders / besser lösen (mit option behälter?)
        }
    }

    fn getPosByAttrname(obj: &GraphObject, attrname: &str) -> i32 {
        if let Some(index) = *obj.keys.iter().position(| &x | x.attrname == attrname) {
            index
        }
        else {
            -1
        }
    }
}