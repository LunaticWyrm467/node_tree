use crate::MutableArc;
use crate::traits::node::DynNode;
use super::{ node_query::NodeQuery, node_tree::NodeTree };



/// Holds all of the node's internal information such as its name, children, parent, and owner.
/// Also allows for the modification of the node's internal state.
pub struct NodeBase {
    name:     String,
    parent:   NodeQuery,
    owner:    NodeQuery,
    root:     Option<MutableArc<NodeTree>>,
    children: Vec<DynNode>
}

impl NodeBase {

    /// Creates a new NodeBase instance with no parent, owner, or root.
    pub fn new(name: String) -> Self {
        NodeBase {
            name,
            parent:   NodeQuery::None,
            owner:    NodeQuery::None,
            root:     None,
            children: Vec::new()
        }
    }

    /// Gets the name of the node.
    /// Each name must be unique within the context of the parent's children vector.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of the node without checking if the name is unique.
    /// This should only be implemented, but not used manually.
    pub unsafe fn set_name_unchecked(&mut self, name: &str) -> () {
        self.name = name.to_string();
    }

    /// Gets the reference to the root NodeTree structure, which controls the entire tree.
    /// This will return None if the node is not connected to the NodeTree.
    pub fn root(&self) -> Option<MutableArc<NodeTree>> {
        self.root.clone()
    }

    /// Sets the reference to the root NodeTree structure.
    pub unsafe fn set_root(&mut self, root: MutableArc<NodeTree>) -> () {
        self.root = Some(root);
    }

    /// Gets the owner of the node.
    /// The owner is different from the parent. The owner can be thought as the root of the scene
    /// that this node is a part of, rather than the node's actual parent.
    /// In other words, if you had a node tree that looked like this:
    /// ```text
    /// ... <Higher Nodes>
    /// ╰NodeA <Root of Saved Scene>
    ///  ├NodeB
    ///  ╰NodeC
    ///   ╰NodeD
    ///```
    /// And you were to call `owner()` on `NodeD`, you would get `NodeA`.
    /// # Note
    /// You can only have an owner on a node that is a part of a node tree.
    pub fn owner(&self) -> NodeQuery {
        self.owner.clone()
    }

    /// Sets the owner of the node.
    /// This should only be implemented, but not used manually.
    pub unsafe fn set_owner(&mut self, owner: DynNode) -> () {
        self.owner = NodeQuery::Some(owner);
    }

    /// Gets the direct parent of this node.
    pub fn parent(&self) -> NodeQuery {
        self.parent.clone()
    }

    /// Sets the parent of this node.
    /// This should only be implemented, but not used manually.
    pub unsafe fn set_parent(&mut self, parent: DynNode) -> () {
        self.parent = NodeQuery::Some(parent);
    }

    /// Gets a vector of this node's children.
    pub fn children(&self) -> &Vec<DynNode> {
        &self.children
    }
    
    /// Gets a mutable vector of this node's children.
    pub fn children_mut(&mut self) -> &mut Vec<DynNode> {
        &mut self.children
    }
}
