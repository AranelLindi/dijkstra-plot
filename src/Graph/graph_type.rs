use std::str::FromStr;
use crate::Graph::graph_type::graph_enum::GraphType;

pub mod graph_enum {
    #[derive(Clone, PartialEq)] // TODO: Is Clone-Trait necessary here?
    pub enum GraphType {
        Directed,
        Undirected
    }
}

impl FromStr for GraphType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s { 
            "true" => Ok(GraphType::Directed),
            "false" => Ok(GraphType::Undirected),
            _ => Err(format!("unknown graph type: {}", s))
        }
    }
}