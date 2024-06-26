//===================================================================================================================================================================================//
//
//  /$$   /$$                 /$$                 /$$$$$$$                               
// | $$$ | $$                | $$                | $$__  $$                              
// | $$$$| $$  /$$$$$$   /$$$$$$$  /$$$$$$       | $$  \ $$  /$$$$$$   /$$$$$$$  /$$$$$$ 
// | $$ $$ $$ /$$__  $$ /$$__  $$ /$$__  $$      | $$$$$$$  |____  $$ /$$_____/ /$$__  $$
// | $$  $$$$| $$  \ $$| $$  | $$| $$$$$$$$      | $$__  $$  /$$$$$$$|  $$$$$$ | $$$$$$$$
// | $$\  $$$| $$  | $$| $$  | $$| $$_____/      | $$  \ $$ /$$__  $$ \____  $$| $$_____/
// | $$ \  $$|  $$$$$$/|  $$$$$$$|  $$$$$$$      | $$$$$$$/|  $$$$$$$ /$$$$$$$/|  $$$$$$$
// |__/  \__/ \______/  \_______/ \_______/      |_______/  \_______/|_______/  \_______/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! The `NodeBase` acts as the core of your `Node` types.
//! It handles storing references to children, parents, managing data, etc.
//!
//! Every `Node` type must contain a `base: Rc<NodeBase>` field for this reason.
//!

use std::rc::Rc;

use crate::traits::node::DynNode;
use super::{ high_pointer::Hp, /*node_query::NodeQuery,*/ node_tree::NodeTree };


#[derive(Debug, Clone)]
pub enum NodeStatus {
    Normal,
    JustWarned(String),
    JustPanicked(String)
}

/// Holds all of the node's internal information such as its name, children, parent, and owner.
/// Also allows for the modification of the node's internal state.
/// # Note
/// This does not derive from the Debug macro, but rather implements Debug manually to avoid
/// issues with recursion whilst debug printing.
pub struct NodeBase {
    name:      String,
    unique_id: String,
    parent:    Option<DynNode>,
    owner:     Option<DynNode>,
    root:      Option<Hp<NodeTree>>,
    children:  Vec<DynNode>,
    status:    NodeStatus,
    depth:     usize   // How far the Node is within the tree.
}

impl NodeBase {

    /// Creates a new NodeBase instance with no parent, owner, or root.
    pub fn new(name: String) -> Rc<Self> {
        Rc::new(NodeBase {
            name,
            unique_id: String::new(),
            parent:    None,
            owner:     None,
            root:      None,
            children:  Vec::new(),
            status:    NodeStatus::Normal,
            depth:     0
        })
    }

    /// Gets the name of the node.
    /// Each name is unique within the context of the parent's children vector.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of the node without checking if the name is unique.
    /// This should only be implemented, but not used manually.
    pub unsafe fn set_name_unchecked(&mut self, name: &str) -> () {
        self.name = name.to_string();
    }

    /// Gets the unique ID of the node.
    /// Each unique ID is unique within the context of the entire NodeTree.
    pub fn unique_id(&self) -> &str {
        &self.unique_id
    }

    /// Sets the unique ID of the node.
    /// This should only be implemented, but not used manually.
    pub unsafe fn set_unique_id(&mut self, unique_id: String) -> () {
        self.unique_id = unique_id;
    }

    /// Gets the reference to the root NodeTree structure, which controls the entire tree.
    /// This will return None if the node is not connected to the NodeTree.
    pub fn root(&self) -> Option<Hp<NodeTree>> {
        self.root
    }

    /// Sets the reference to the root NodeTree structure.
    pub unsafe fn set_root(&mut self, root: Hp<NodeTree>) -> () {
        self.root = Some(root);
    }

    /// Disconnects the NodeTree from this node.
    /// This should only be implemented, but not used manually.
    pub unsafe fn disconnnect_root(&mut self) -> () {
        self.root = None;
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
    pub fn owner(&self) -> Option<DynNode> {
        self.owner
    }

    /// Sets the owner of the node.
    /// This should only be implemented, but not used manually.
    pub unsafe fn set_owner(&mut self, owner: DynNode) -> () {
        self.owner = Some(owner);
    }

    /// Disconnects this node's owner from this node.
    /// This should only be implemented, but not used manually.
    pub unsafe fn disconnnect_owner(&mut self) -> () {
        self.owner = None;
    }

    /// Gets the direct parent of this node.
    pub fn parent(&self) -> Option<DynNode> {
        self.parent
    }

    /// Sets the parent of this node.
    /// This should only be implemented, but not used manually.
    pub unsafe fn set_parent(&mut self, parent: DynNode) -> () {
        self.parent = Some(parent);
    }

    /// Disconnects this node's parent from this node.
    /// This should only be implemented, but not used manually.
    pub unsafe fn disconnnect_parent(&mut self) -> () {
        self.parent = None;
    }

    /// Gets a vector of this node's children.
    pub fn children(&self) -> &Vec<DynNode> {
        &self.children
    }
    
    /// Gets a mutable vector of this node's children.
    pub fn children_mut(&mut self) -> &mut Vec<DynNode> {
        &mut self.children
    }

    /// Gets the node's status.
    pub fn status(&self) -> &NodeStatus {
        &self.status
    }

    /// Sets the node's status.
    /// This should only be implemented, but not used manually.
    pub unsafe fn set_status(&mut self, status: NodeStatus) -> () {
        self.status = status;
    }

    /// Gets the node's depth.
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Sets the node's depth.
    /// This should only be implemented, but not used manually.
    pub unsafe fn set_depth(&mut self, depth: usize) -> () {
        self.depth = depth;
    }
}

impl std::fmt::Debug for NodeBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Inner[{}] {{ ", self.name))?;
        
        if let Some(parent) = &self.parent {
            f.write_str(&format!("Parent: {}, ", parent.name()))?;
        }
        if let Some(owner) = &self.owner {
            f.write_str(&format!("Owner: {}, ", owner.name()))?;
        }

        f.write_str(&format!("Connected to Tree: {}, ", self.root.is_some()))?;
        f.write_str(&format!("Children: {:?}, ", &self.children))?;
        f.write_str(&format!("Depth: {} ", self.depth))?;
        f.write_str("}")?;

        Ok(())
    }
}
