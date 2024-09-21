use super::node::Node;



/// This marks any object that can be referenced in the `NodeTree` as either a node or a collection
/// of nodes.
pub trait Instanceable {
    
    /// Goes through and iterates through all of the nodes that are represented by this collection.
    /// The arguments passed through are the pointers to the parent (if there is one) and the node.
    fn iterate<F: FnMut(Option<*mut dyn Node>, *mut dyn Node)>(self, iterator: F);
}
