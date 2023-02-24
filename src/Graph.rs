use crate::Graph::node::Node;
use crate::Graph::edge::Edge;
use crate::Graph::key::Key;

mod graphtype;
mod igraph_object;
mod key_type;
mod node;
mod edge;
mod key;

pub struct Graph<'a, 'b> {
    id: &'a str, /* 'id: str<'a>' alternative syntax (if it were not a reference to str). Equivalent to 'id: &'a str'. Useful syntax if no reference is needed but a lifetime ! */
    nodes: Vec<Node<'a, 'b>>, // TODO: Not sure if it is correct. Nodes must live until as long as edges need reference on them which is 'b lifetime... node need only lifetime for id (str) and 'b is given them according to above syntax rule (if compiler accepts)
    edges: Vec<Edge<'a, 'b>>,
    keys: Vec<Key>,
}

impl<'a, 'b> Graph<'a, 'b> {
    // Constructor
    pub fn new(id: &'a str, nodes: Vec<node<'a, 'b>>, edges: Vec<edge<'a, 'b>>, keys: Vec<key>) -> Self {
        Self {
            id,
            nodes,
            edges,
            keys,
        }
    }

    pub fn get_adjacency_matrix(&self) -> &'static Box<[[bool]]> /* return value: 2d array on heap (boxed) with lifetime forever (static) (static because it won't change anymore and to test and bypass static lifetime) */
    {
        let N: u32 = self.nodes.len() as u32;

        // creates 2D array respectively N x N matrix within N := len(self.nodes) (vec<bool> would probably be more efficient but I'm going to use an array here anyway (learning process))
        let mut matrix = [[false; N]; N]; // TODO: Array works only with const value, so switch to vector then !

        // enumerate returns tuple with (position, current object)
        for (_, e) in self.edges.iter().enumerate() {
            matrix[e.source.no][e.dest.no] = true;
        }

        // Boxing the matrix and return it
        Box::leak(matrix.into_boxed_slice())
    }

    pub fn add_key(obj: &mut dyn Igraph_Object, key: &key) {
        // find out if key already exists
        if let Some(index) = obj.keys.iter().position(|&x| *x.getID() == key.id) {
            // element exists so update value
            obj.keys[index].value = key.value;
        } else {
            // element is not yet part of vector, so add it
            obj.keys.push(key);
        }
    }

    pub fn delete_key(obj: &dyn Igraph_Object, id: &str) {
        if let Some(index) = *obj.keys.iter().position(|&x| *x.id == *id) {
            // remove object
            obj.keys.remove(index);
        } // (else: element not found in vector so nothing needs to be done)
    }

    pub fn get_pos_key_by_id(obj: &dyn Igraph_Object, id: &str) -> Result<u32, String> {
        if let Some(index) = *obj.keys.iter().position(|&x| *x.id == *id) {
            Ok(index)
        } else {
            Err("Key not found!".to_string())
        }
    }

    pub fn get_pos_by_attrname(obj: &dyn Igraph_Object, attrname: &str) -> Result<u32, String> {
        if let Some(index) = *obj.keys.iter().position(|&x| *x.attrname == attrname) {
            Ok(index)
        } else {
            Err("Key not found!".to_string())
        }
    }
}

impl<'a, 'b> Igraph_Object for Graph<'a, 'b> {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_keys(&self) -> &vec<key> {
        &self.keys
    }
}
