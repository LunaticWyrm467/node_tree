use std::collections::VecDeque;

/// A NodePath is a specialized string that holds a map for the NodeTree to follow and to retrieve
/// a given node.
#[derive(Debug, Clone)]
pub struct NodePath {
    path: VecDeque<String>
}

impl NodePath {

    /// Creates an empty NodePath.
    pub fn new() -> NodePath {
        NodePath {
            path: VecDeque::new()
        }
    }
    
    /// Creates a new NodePath from a string.
    /// In the string, each node name must be seperated by a `/`, like so:
    /// ```text
    /// "node_a/node_b/node_c/target_node"
    /// ```
    pub fn from_str(str: &str) -> NodePath {
        NodePath {
            path: str.split('/').map(|s| s.to_string()).collect()
        }
    }

    /// Adds a node to the back of the path.
    pub fn add_node(&mut self, node_name: &str) -> () {
        self.path.push_back(node_name.to_string())
    }

    /// Pops the front-most node off the path and returns it, if there is one.
    /// If the path is empty, then this returns None.
    pub fn pop_front(&mut self) -> Option<String> {
        self.path.pop_front()
    }
}
