pub mod edge;
pub mod graph_type;
pub mod igraph_object;
pub mod key;
pub mod key_type;
pub mod node;

use std::rc::Rc;
pub use crate::Graph::node::Node;
pub use crate::Graph::edge::Edge;
use crate::Graph::graph_type::graph_enum::GraphType;
pub use crate::Graph::key::Key;
pub use crate::Graph::igraph_object::IgraphObject;
//use crate::GraphML::key_for::KeyFor::Edge;

//#[derive(Hash)]
pub struct Graph<'a> {
    id: &'a str, /* 'id: str<'a>' alternative syntax (if it were not a reference to str). Equivalent to 'id: &'a str'. Useful syntax if no reference is needed but a lifetime ! */
    pub nodes: Vec<Rc<Node<'a>>>, // TODO: Not sure if it is correct. Nodes must live until as long as edges need reference on them which is 'b lifetime... node need only lifetime for id (str) and 'b is given them according to above syntax rule (if compiler accepts)
    pub edges: Vec<Rc<Edge<'a>>>,
    keys: Vec<Key<'a>>,
    pub node_len: usize,
    pub edge_len: usize
    //marker: std::marker::PhantomData<&'a &'b()>
}

impl<'a> Graph<'a> {
    // TODO: new2 is only temporarily implemented!
    pub fn new(id: &'a str, nodes: Vec<Rc<Node<'a>>>, edges: Vec<Rc<Edge<'a>>>, keys: Vec<Key<'a>>) -> Self {
        // Necessary to store it here because by initiate Self nodes and edges are moved and there is no access to len() anymore
        let node_len = nodes.len();
        let edge_len = edges.len();

        Self {
            id,
            nodes,
            edges,
            keys,
            node_len,
            edge_len,
        }
    }

    // Constructor
/*    pub fn new(id: String, nodes_info: Vec<(String, Vec<Key>, usize)>, edges_info: Vec<(String, u32, GraphType, usize, usize, Vec<Key>)>, keys: Vec<Key>) -> Result<Self, String> {
        // Necessary to store it here because by initiate Self nodes and edges are moved and there is no access to len() anymore

/*        let nodes: Vec<Rc<Node>> = nodes_info
            .into_iter()
            .map(|(id, keys, no| Rc::new(Node::new(id, keys, no)))
                .collect();*/


    }*/

    // return type is immutable because '&self' parameter! To be mutable, it must be '&mut self'. Function returns a boxed matrix
    // which is stored on heap and valid until Graph object is alive (I want it like this!)
    pub fn get_adjacency_matrix(&self) -> Vec<Vec<bool>>
    {
        // creates N x N matrix within N := self.nodes.len() of type bool
        let mut matrix: Vec<Vec<bool>> = vec![vec![false; self.node_len()]; self.node_len()];

        // enumerate returns tuple with (position, current object)
        for (_, e) in self.edges.iter().enumerate() {
            matrix[e.source().no()][e.dest().no()] = true;
        }

        return matrix;
    }

    /* obj: &'a mut (dyn IgraphObject + 'a) means:
        &'a mut : mutable reference valid as long as 'a lives to ...
        (dyn IgraphObject + 'a) : an object which implements the IgraphObject trait and exists at least as long 'a does (+ is used to connect constraints)
     */
    pub fn add_key(obj: &'a mut (dyn IgraphObject<'a> + 'a), key: Key<'a>) {
        let keys: &'a mut Vec<Key> = obj.get_keys();//.to_vec().clone();

        // find out if key already exists
        if let Some(index) = keys.iter().position(|x| x.id == key.id) {
            // element exists so update value
            keys[index].default = key.default;
        } else {
            keys.push(key);
        }
    }

    pub fn delete_key(obj: &'a mut (dyn IgraphObject<'a> + 'a), id: &'a str) {
        let keys: &'a mut Vec<Key> = obj.get_keys();

        if let Some(index) = keys.iter().position(|x| x.id == id) {
            // remove object
            keys.remove(index);
        } // (else: element not found in vector so nothing needs to be done)
    }

    pub fn get_pos_key_by_id(obj: &'a mut (dyn IgraphObject<'a> + 'a), id: &'a str) -> Result<usize, String> {
        if let Some(index) = obj.get_keys().iter().position(|x| x.id == id) {
            Ok(index)
        } else {
            Err("Key not found!".to_string())
        }
    }

    pub fn get_pos_by_attrname(obj: &'a mut (dyn IgraphObject<'a> + 'a), attrname: &'a str) -> Result<usize, String> {
        if let Some(index) = obj.get_keys().iter().position(|x| x.attrname == attrname) {
            Ok(index)
        } else {
            Err("Key not found!".to_string())
        }
    }
    pub fn node_len(&self) -> usize {
        self.node_len
    }
    pub fn edge_len(&self) -> usize {
        self.edge_len
    }
}

impl<'a> IgraphObject<'a> for Graph<'a> {
    fn get_id(&'a self) -> &'a str {
        &self.id
    }
    fn get_keys(&mut self) -> &mut Vec<Key<'a>> {
        &mut self.keys
    }
    fn set_keys(&'a mut self, keys: Vec<Key<'a>>) {
        self.keys = keys;
    }
}