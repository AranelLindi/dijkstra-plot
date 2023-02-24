use crate::Graph::key::Key;
use crate::Graph::igraph_object::IgraphObject;

pub struct Node<'a, 'b> {
    id: &'a str, /* It would also be possible to use a static lifetime here. That would be mean that the object is deallocated when the node object is deallocated. Lifetime 'a makes sure that the variable is deallocated at earliest it is not needed any more */
    keys: Vec<key>,
    pub(crate) no: u32
}

impl<'a> IgraphObject for Node<'a> {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_keys(&self) -> &Vec<key> {
        &self.keys
    }
}