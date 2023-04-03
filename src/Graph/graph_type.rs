pub mod graph_enum {
    #[derive(Clone, PartialEq)] // TODO: Is Clone-Trait necessary here?
    pub enum GraphType {
        Directed,
        Undirected
    }
}