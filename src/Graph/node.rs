use crate::Graph::igraph_object::IgraphObject;
use crate::Graph::key::Key;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Node {
    id: String, /* It would also be possible to use a static lifetime here. That would be mean that the object is deallocated when the node object is deallocated. Lifetime 'a makes sure that the variable is deallocated at earliest when it is not longer needed */
    keys: Vec<Key>,
    no: u32,
    //marker: std::marker::PhantomData<&'a &'b()> /* Phantom Object: Contains no data, but is useful to convince the compiler that lifetime requirements are met. In this case: Node must be at least live as long as Edge lives. Edge got the lifetime parameter 'b for its references to Node, so Node must use 'b too! */
}

impl Node {
    pub fn no(&self) -> u32 {
        self.no
    }

    pub fn new(id: String, keys: Vec<Key>, no: u32) -> Self {
        Self {
            id,
            keys,
            no
        }
    }
}

impl<'a> IgraphObject<'a> for Node {
    fn get_id(&'a self) -> &'a str {
        self.id.as_str()
    }

    fn get_keys(&mut self) -> &mut Vec<Key> {
        &mut self.keys
    }

    fn set_keys(&mut self, keys: Vec<Key>) {
        self.keys = keys;//.to_vec();
    }
}