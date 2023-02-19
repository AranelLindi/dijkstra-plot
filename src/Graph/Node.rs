struct Node {
    id: String,
    keys: Vec<Key>,
    no: u32
}

impl GraphObject for Node {
    fn getID(&self) -> String {
        self.id
    }

    fn getKeys(&self) -> Vec<Key> {
        self.keys
    }
}