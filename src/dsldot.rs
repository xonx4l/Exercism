pub mod graph {
    use std::collections::Hashmap;

    pub struct Graph {
        pub attrs: Hashmap<String, String>,
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>
    }

    impl Graph {
       pub fn new() -> Self {
            Self { attrs: Hashmap::new(), nodes: Vec::new(), edges: vec::new() }
        }
       pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            attrs.iter().for_each(|(name, value)| {self.attrs.insert(name.to_string), value.to_string());});
            self 
       }
       pub fn with_nodes(mut self, nodes: &Vec<graph_items::node::Node>) -> Self {
           self.nodes = nodes.clone();
           self
      }
    }
}
