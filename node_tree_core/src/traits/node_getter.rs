use crate::structs::{ node_tree::NodeTree, rid::RID };


/// A trait that is implemented for types that can be used to get node RIDs from the `NodeTree`.
pub trait NodeGetter {
    
    /// A function that must be implemented per compatible type.
    fn get_from(&self, tree: &NodeTree) -> Option<RID>;
}
