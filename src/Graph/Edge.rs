mod enum_module {
    pub use super::graphtype;
}

pub struct Edge {
    id: String,
    pub weight: u32,
    pub etype: enum_module::graphtype,
    pub source: &Node,
    pub dest: &Node,
    keys: Vec<Key>
}

impl GraphObject for Edge {
    fn getID(&self) -> String {
        self.id
    }

    fn getKeys(&self) -> Vec<Key> {
        self.keys
    }
}