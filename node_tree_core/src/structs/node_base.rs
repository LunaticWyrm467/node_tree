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

use std::{ rc::Rc, sync::Mutex };

use super::{
    logger::Log,
    node_path::NodePath,
    node_scene::NodeScene,
    node_tree_base::{ NodeTreeBase, TerminationReason },
    tree_pointer::{ Tp, TpDyn },
    tree_result::TreeResult,
    rid::RID
};

use crate::traits::{ node::Node, node_tree::NodeTree, node_getter::NodeGetter, instanceable::Instanceable };
use crate::utils::functions::ensure_unique_name;


#[derive(Debug, Clone)]
pub enum NodeStatus {
    Normal,
    JustWarned(String),
    JustPanicked(String)
}

/// Holds all of the node's internal information such as its name, children, parent, owner, and
/// owning `NodeTree`.
/// Also allows for the modification of the node's internal state.
///
/// # Note
/// Cloning this will result in a new `NodeBase` with the same name.
pub struct NodeBase {
    name:     String,
    rid:      RID,
    parent:   Option<RID>,
    owner:    Option<RID>,
    tree:     Option<*mut dyn NodeTree>,  // Lifetimes are managed by the NodeTree/Nodes
    children: Vec<RID>,
    status:   Rc<Mutex<NodeStatus>>,
    loaded:   bool,
    depth:    usize   // How far the Node is within the tree.
}

impl NodeBase {

    /// Creates a new `NodeBase` instance with no parent, owner, or owning tree.
    pub fn new(name: String) -> Self {
        NodeBase {
            name,
            rid:      RID::default(),
            parent:   None,
            owner:    None,
            tree:     None,
            children: Vec::new(),
            status:   Rc::new(Mutex::new(NodeStatus::Normal)),
            loaded:   false,
            depth:    0
        }
    }
    
    /// Returns a `Tp<T>` pointer for itself.
    ///
    /// # Panics
    /// The correct type (type of `self`) will have to be provided, or this function will panic!
    /// Panics also if this Node is not connected to a `NodeTree`.
    pub fn this<S: Node>(&self) -> Tp<S> {
        if self.tree.is_none() {
            panic!("Cannot get a tree pointer to a node that is not in a `NodeTree`!");
        }

        unsafe {
            Tp::new(self.tree.unwrap_unchecked(), self.rid, self.rid).expect("Called `this` with the wrong type provided")
        }
    }
    
    /// Returns a `TpDyn` pointer for itself.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn this_dyn(&self) -> TpDyn {
        if self.tree.is_none() {
            panic!("Cannot get a tree pointer to a node that is not in a `NodeTree`!");
        }
        
        // SAFETY:
        // There are no invariants that could result in this being unwrapped on an `Err` value.
        unsafe {
            TpDyn::new(self.tree.unwrap_unchecked(), self.rid, self.rid).unwrap_unchecked()
        }
    }

    /// Gets the name of the node.
    /// Each name is unique within the context of the parent's children vector.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of the node.
    /// If the name is not unique among the node's siblings, then it will be made into a unique name.
    pub fn set_name(&mut self, name: &str) {
        if let (Some(parent), Some(tree)) = (self.parent, self.tree()) {
            let     parent:    &dyn Node    = unsafe { tree.get_node(parent).unwrap_unchecked() };
            let     siblings:  &[String]    = &parent.children().iter().map(|a| a.name().to_string()).collect::<Vec<_>>();

            unsafe {
                self.set_name_unchecked(&ensure_unique_name(name, siblings));
            }
        } else {
            unsafe {
                self.set_name_unchecked(name);
            }
        }
    }

    /// Registers this node as a singleton.
    /// Returns whether the name was set successfully.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn register_as_singleton(&mut self, name: String) -> bool {
        let rid: RID = self.rid;
        match self.tree_mut() {
            None       => panic!("Cannot register a node that is not apart of the Nodetree as a singleton!"),
            Some(tree) => tree.register_as_singleton(rid, name).unwrap()
        }
    }

    /// Adds a child to the node, automatically renaming it if its name is not unique in the
    /// node's children vector.
    ///
    /// # Note
    /// `_ready()` will automatically be propogated through the added child node.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn add_child<I: Instanceable>(&mut self, child: I) {
        child.iterate(|parent, node, is_owner| {
            if let Some(parent) = parent {
                unsafe {
                    let parent: &mut dyn Node = &mut *parent;
                    parent.add_child_from_ptr(node, is_owner, false);
                }
            } else {
                unsafe {
                    self.add_child_from_ptr(node, is_owner, false);
                }
            }
        });
    }

    /// Adds a child to the node via a passed in pointer, automatically renaming it if its
    /// name is not unique in the node's children vector.
    ///
    /// Returns the child's new RID.
    ///
    /// # Arguments
    /// Aside from the raw pointer to the child itself, this function takes in two booleans for
    /// whether if this node marks the owner of a new scene branch, and if this added node does not
    /// call its ready() function respectively.
    ///
    /// # Note
    /// `_ready()` will automatically be propogated through the added child node.
    ///
    /// # Safety
    /// Cannot guarantee that the raw pointer that is passed in is valid.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub unsafe fn add_child_from_ptr(&mut self, child_ptr: *mut dyn Node, owner_is_self: bool, ignore_ready: bool) -> RID {
        if self.tree.is_none() {
            panic!("Cannot add a child to a node that is not in a `NodeTree`!");
        }

        // Ensure that the child's name within the context of this node's children is unique.
        let names_of_children: &[String] = &self.children().iter().map(|c| c.name().to_string()).collect::<Vec<_>>();
        let child_name:        &str      = unsafe { &*child_ptr }.name();
        let unique_name:       String    = ensure_unique_name(child_name, names_of_children);

        // Add the child to this node's children and connect it to its parent and owner nodes,
        // as well as the root tree structure's reference.
        let child_rid: RID = unsafe {
            let owner_rid:  RID               = self.owner.unwrap_unchecked();
            let parent_rid: RID               = self.rid;
            let new_depth:  usize             = self.depth() + 1; 
            let tree_raw:   *mut dyn NodeTree = self.tree.unwrap_unchecked();
            let tree:       &mut dyn NodeTree = self.tree_mut().unwrap_unchecked();
            
            let rid:   RID           = tree.register_node(child_ptr);
            let child: &mut dyn Node = tree.get_node_mut(rid).unwrap_unchecked();

            child.set_name_unchecked(&unique_name);
            child.set_parent(parent_rid);
            child.set_owner(if owner_is_self { rid } else { owner_rid });
            child.set_tree(tree_raw);
            child.set_depth(new_depth);   // This is the only place where depth is updated.
            
            child.set_rid(rid);
            rid
        };
        self.children.push(child_rid);
        
        // Call the `ready()` function for the child as long as the call to ready() is not ignored
        // or circumvented..
        if !ignore_ready {
            unsafe {
                let child: &mut dyn Node = self.tree_mut().unwrap_unchecked().get_node_mut(child_rid).unwrap_unchecked();
                if child.has_just_loaded() {
                    child.loaded();
                    child.mark_as_final();
                }
                child.ready();

            }
        }
        
        // Print the debug information on the child to the console.
        let child: &dyn Node = unsafe { self.tree().unwrap_unchecked().get_node(child_rid).unwrap_unchecked() };
        self.post(Log::Debug(&format!("Node \"{}\" added to the scene as the child of \"{}\"! Unique ID of \"{}\" generated!", child.name(), self.name(), child.rid)));
        child_rid
    }

    /// Removes a child but it does not destroy it, disconnecting from its parent.
    /// Both the child and its children will be disconnected from the tree and their owners.
    /// This will return whether the child node was successfully removed or not.
    ///
    /// # Note
    /// This will result in all removed nodes having their `terminal()` function called with the
    /// reason `RemovedAsChild`.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn remove_child(&mut self, name: &str) -> bool {
        if self.tree.is_none() {
            panic!("Cannot add a child to a node that is not in a `NodeTree`!");
        }

        // TODO:
        // This function could be cleaned up a bit...
        
        // Locate a child node that has the same name. If there is no matching node, then exist
        // early.
        let child: Option<(usize, TpDyn)> = self.children()
            .into_iter()
            .enumerate()
            .find(|(_, c)| c.name() == name);
        if child.is_none() {
            self.post(Log::Warn(&format!("Attempted to remove invalid node of name \"{}\" from node \"{}\"!", name, self.name())));
            return false;
        }

        let (
            child_idx,
            child_name,
            connected
        ): (usize, String, Vec<RID>) = unsafe {
            child.map(|(idx, child)| (
                    idx,
                    child.name().to_string(),
                    child.top_down(true)
            )).unwrap_unchecked()
        };

        self.children.remove(child_idx);
        for (idx, queued_rid) in connected.into_iter().enumerate() { unsafe { 
            let _is_root_child: bool          = idx == 0; // TODO: Use this to save children nodes!
            let queued_node:    &mut dyn Node = self.tree_mut().unwrap_unchecked().get_node_mut(queued_rid).unwrap_unchecked();
            
            queued_node.terminal(TerminationReason::RemovedAsChild);
            queued_node.disconnnect_parent();
            queued_node.disconnnect_owner();
            queued_node.disconnnect_tree();

            self.tree_mut().unwrap_unchecked().unregister_node(queued_rid);
        }}

        self.post(Log::Debug(&format!("Removed child node \"{}\" from parent node \"{}\"!", child_name, self.name())));
        true 
    }

    /// Returns a `Tp<T>` pointer to a child at the given index.
    /// If there is no child at the given index, or if the wrong type is given, then `Err` will be returned.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn get_child<T: Node>(&self, i: usize) -> TreeResult<Tp<T>> {
        if self.tree().is_none() {
            panic!("Cannot get a node from a node that is not a part of a NodeTree!");
        }

        if i >= self.num_children() {
            unsafe {
                TreeResult::new(self.tree.unwrap_unchecked(), self.rid, Err(format!("Index {i} was out of the range of this node's children count of {}", self.num_children())))
            }
        } else {
            unsafe {
                Tp::new(self.tree.unwrap_unchecked(), self.rid, self.children[i])
            }
        }
    }
    
    /// Returns a `TpDyn` pointer to a child at the given index.
    /// If there is no child at the given index then `Err` will be returned.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn get_child_dyn(&self, i: usize) -> TreeResult<TpDyn> {
        if self.tree().is_none() {
            panic!("Cannot get a node from a node that is not a part of a NodeTree!");
        }

        if i >= self.num_children() {
            unsafe {
                TreeResult::new(self.tree.unwrap_unchecked(), self.rid, Err(format!("Index {i} was out of the range of this node's children count of {}", self.num_children())))
            }
        } else {
            unsafe {
                TpDyn::new(self.tree.unwrap_unchecked(), self.rid, self.children[i])
            }
        }
    }

    /// Gets a vector of `DynTp` to describe this node's children.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn children(&self) -> Vec<TpDyn> {
        if self.tree().is_none() {
            panic!("Cannot get children from a node that is not a part of a NodeTree!");
        }

        self.children.iter().map(|&c| unsafe { TpDyn::new(self.tree.unwrap_unchecked(), self.rid, c).unwrap_unchecked() }).collect()
    }

    /// Gets a `Tp<T>` or a Tree Pointer to a given `Node` via a `NodePath`.
    /// Returns `Err` if the address is invalid or if the referenced `Node` is not of the type
    /// `T`.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn get_node<T: Node>(&self, path: NodePath) -> TreeResult<Tp<T>> {
        if self.tree().is_none() {
            panic!("Cannot get a node from a node that is not a part of a NodeTree!");
        }

        match self.get_node_raw(path.clone()) {
            Some(node_rid) => {
                unsafe {
                    Tp::new(self.tree.unwrap_unchecked(), self.rid, node_rid)
                }
            },
            None => unsafe {
                TreeResult::new(self.tree.unwrap_unchecked(), self.rid, Err(format!("The path {path:?} is invalid")))
            }
        }
    }

    /// Gets a `TpDyn` or a Dynamic Tree Pointer to a given `Node` via a `NodePath`.
    /// Returns `Err` if the address is invalid.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn get_node_dyn(&self, path: NodePath) -> TreeResult<TpDyn> {
        if self.tree().is_none() {
            panic!("Cannot get a node from a node that is not a part of a NodeTree!");
        }

        match self.get_node_raw(path.clone()) {
            Some(node_rid) => {
                unsafe {
                    TpDyn::new(self.tree.unwrap_unchecked(), self.rid, node_rid)
                }
            },
            None => unsafe {
                TreeResult::new(self.tree.unwrap_unchecked(), self.rid, Err(format!("The path {path:?} is invalid")))
            }
        }
    }
    
    /// Gets a `Tp<T>` or a Tree Pointer to a given `Node` via either a `NodePath`, a `&str`, or a
    /// String (the latter two may be used to denote Singletons).
    /// Returns `Err` if the address is invalid or if the referenced `Node` is not of the type
    /// `T`.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn get_node_from_tree<T: Node, G: NodeGetter>(&self, path: G) -> TreeResult<Tp<T>> {
        if self.tree().is_none() {
            panic!("Cannot get a node from a node that is not a part of a NodeTree!");
        }
        let path_str: String = format!("{path:?}");

        match unsafe { self.tree().unwrap_unchecked() }.get_node_rid(path) {
            Some(node_rid) => {
                unsafe {
                    Tp::new(self.tree.unwrap_unchecked(), self.rid, node_rid)
                }
            },
            None => unsafe {
                TreeResult::new(self.tree.unwrap_unchecked(), self.rid, Err(format!("The root path {path_str:?} is invalid")))
            }
        }
    }

    /// Gets a `TpDyn` or a Dynamic Tree Pointer to a given `Node` via either a `NodePath`, a `&str`, or a
    /// String (the latter two may be used to denote Singletons).
    /// Returns `Err` if the address is invalid.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn get_node_dyn_from_tree<G: NodeGetter>(&self, path: G) -> TreeResult<TpDyn> {
        if self.tree().is_none() {
            panic!("Cannot get a node from a node that is not a part of a NodeTree!");
        }
        let path_str: String = format!("{path:?}");

        match unsafe { self.tree().unwrap_unchecked() }.get_node_rid(path) {
            Some(node_rid) => {
                unsafe {
                    TpDyn::new(self.tree.unwrap_unchecked(), self.rid, node_rid)
                }
            },
            None => unsafe {
                TreeResult::new(self.tree.unwrap_unchecked(), self.rid, Err(format!("The root path {path_str:?} is invalid")))
            }
        }
    }

    /// Gets a node's `RID` given a `NodePath` that is respective to this node as the root.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn get_node_raw(&self, mut path: NodePath) -> Option<RID> {
        if self.tree().is_none() {
            panic!("Cannot get a node from a node that is not a part of a NodeTree!");
        }

        let next_node: Option<String> = path.pop_front();
        match next_node {
            Some(target) => {
                for node in self.children() {
                    if node.name() == target {
                        return node.get_node_raw(path);
                    }
                }
                None
            },
            None => Some(self.rid())
        }
    }

    /// Produces a normal top-down order iteration of all of the nodes connected to this node.
    /// This is used to handle a lot of the scene tree behaviour.
    /// If 'contains_self' is true, then the list will contain this node as well.
    ///
    /// # Note
    /// Nodes that are at the beginning of the children vector will be prioritized.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn top_down(&self, contains_self: bool) -> Vec<RID> {
        let mut iter: Vec<RID> = if contains_self { vec![self.rid()] } else { Vec::new() };
        self.top_down_tail(&mut iter, vec![self.rid()]);
        iter
    }

    /// The tail end recursive function for the `top_down` method.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    fn top_down_tail(&self, iter: &mut Vec<RID>, layer: Vec<RID>) {
        if self.tree().is_none() {
            panic!("Cannot get nodes from a node that is not a part of a NodeTree!");
        }
        
        let new_layer: Vec<RID> = unsafe { self.tree().unwrap_unchecked() }.get_all_valid_nodes(&layer)
            .into_iter()
            .flat_map(|node| node.children.to_owned() )
            .collect();
        if new_layer.is_empty() {
            return;
        }
        iter.append(&mut new_layer.clone());

        self.top_down_tail(iter, new_layer)        
    }

    /// Gets this Node's absolute `NodePath` to the root of the tree.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn get_absolute_path(&self) -> NodePath {
        let mut path: String = String::new();
        self.get_absolute_path_tail(&mut path);
        NodePath::from_str(&path)
    }

    /// The recursive tail for the `get_absolute_path` function.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    fn get_absolute_path_tail(&self, path: &mut String) {
        if self.tree().is_none() {
            panic!("Cannot get nodes from a node that is not a part of a NodeTree!");
        }

        *path = self.name().to_string() + &(if path.is_empty() { String::new() } else { "/".to_string() + path });
        if !self.is_root() {
            unsafe {
                self.tree().unwrap_unchecked().get_node(self.parent.unwrap_unchecked()).unwrap_unchecked().get_absolute_path_tail(path);
            }
        }
    }

    /// Attempts to post a log to the logger.
    /// If this node has a unique identifier accessible by name, then that will be used as the
    /// node's identifier in the log.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn post(&self, log: Log) {
        unsafe {
            match &log {
                Log::Warn(str)  => self.set_status(NodeStatus::JustWarned(str.to_string())),
                Log::Panic(str) => self.set_status(NodeStatus::JustPanicked(str.to_string())),
                _               => ()
            }
        }

        let rid: RID = self.rid();
        match self.tree_mut() {
            Some(root) => {
                root.post(rid, log);
            },
            None => panic!("Cannot post to log on a disconnected node!")
        }
    }

    /// Destroys the Node, removing it from any connected parent or children.
    /// If this is the root node, then the destruction of this node will result in the program
    /// itself terminating.
    ///
    /// # Note
    /// This will result in all removed nodes having their `terminal()` function called with the
    /// reason `Freed`.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn free(&mut self) {
        if self.tree().is_none() {
            panic!("Cannot free a node that is not a part of a NodeTree! Instead, simply let the unbound Node drop out of scope or use drop()!");
        }
        
        // Call the terminal function on this node,
        // before removing it and all it's children nodes from the NodeTree.
        for node in self.top_down(true) {
            let is_self: bool              = node == self.rid;
            let tree:    &mut NodeTreeBase = unsafe { self.tree_mut().unwrap_unchecked() };

            unsafe {
                tree.get_node_mut(self.rid).unwrap_unchecked().terminal(TerminationReason::Freed); // Has to be called externally!
            }

            // Remove the reference of this node from its parent if it has a parent.
            if is_self {
                if let Some(parent) = self.parent {
                    unsafe {
                        let rid:       RID           = self.rid;
                        let parent:    &mut dyn Node = self.tree_mut().unwrap_unchecked().get_node_mut(parent).unwrap_unchecked();
                        let child_idx: usize         = parent.children.iter().position(|&c_rid| c_rid == rid).unwrap_unchecked();

                        parent.children.remove(child_idx);
                    }
                }
            }

            unsafe {
                tree.unregister_node(node);
            }
        }

        // If this is the root node, terminate the NodeTree.
        if self.is_root() {
            self.tree_mut().unwrap().terminate();
        }
    }

    /// Saves this node and all of the nodes below it as a `NodeScene`, which can then be
    /// reinstanced somewhere else OR be written to the disk.
    ///
    /// # Note
    /// All data in every `NodeBase` will either be destroyed or be represented in the `NodeScene`'s
    /// representation.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn save_as_branch(&self) -> NodeScene {
        if self.tree().is_none() {
            panic!("Cannot free a node that is not a part of a NodeTree! Instead, simply let the unbound Node drop out of scope or use drop()!");
        }
        self.save_as_branch_tail()
    }

    /// The recursive tail function for `save_as_branch`.
    fn save_as_branch_tail(&self) -> NodeScene {
        
        // Create the root `NodeScene` structure using this node as the root owner.
        let     root:  Box<dyn Node> = unsafe { (&*self.tree.unwrap_unchecked()).get_node(self.rid).unwrap_unchecked() }.clone_as_instance();
        let mut scene: NodeScene     = NodeScene::new_dyn(root);

        // For each child, append their representation of a node scene.
        for &child in &self.children {
            let child: &dyn Node = unsafe { (&*self.tree.unwrap_unchecked()).get_node(child).unwrap_unchecked() };
            if child.is_owner() {
                scene.append_as_owner(child.save_as_branch_tail());
            } else {
                scene.append(child.save_as_branch_tail());
            }
        }
        scene
    }

    /// Sets the name of the node without checking if the name is unique.
    ///
    /// # Safety
    /// Setting this manually with a name that is already present in another sibling could cause
    /// hard to find bugs.
    pub unsafe fn set_name_unchecked(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Gets the unique `RID` (resource ID) of the node.
    /// Each `RID` is unique within the context of the entire `NodeTree`.
    pub fn rid(&self) -> RID {
        self.rid
    }

    /// Sets the unique `RID` of the node.
    ///
    /// # Safety
    /// This should NOT be called manually.
    pub unsafe fn set_rid(&mut self, rid: RID) {
        self.rid = rid;
    }

    /// Gets a reference to the owning `NodeTree` structure, which controls the entire tree.
    /// This will return `None` if the node is not connected to the `NodeTree`.
    pub fn tree(&self) -> Option<&dyn NodeTree> {
        unsafe {
            self.tree.map(|x| &*x)
        }
    }

    /// Gets a mutable reference to the owning `NodeTree` structure, which controls the entire tree.
    /// This will return `None` if the node is not connected to the `NodeTree`.
    pub fn tree_mut(&self) -> Option<&mut dyn NodeTree> {
        unsafe {
            self.tree.map(|x| &mut *x)
        }
    }

    /// Sets the reference to the owning `NodeTree` structure.
    ///
    /// # Safety
    /// This should NOT be called manually.
    pub unsafe fn set_tree(&mut self, tree: *mut dyn NodeTree) {
        self.tree = Some(tree);
    }

    /// Disconnects the `NodeTree` from this node.
    ///
    /// # Safety
    /// This should NOT be called manually.
    pub unsafe fn disconnnect_tree(&mut self) {
        self.tree = None;
    }

    /// Gets the `Tp<T>` owner of the node. Returns `Err` if `T` does not match the owner's type.
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
    ///
    /// # Note
    /// You can only have an owner on a node that is a part of a node tree.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn owner<T: Node>(&self) -> TreeResult<Tp<T>> {
        if self.tree().is_none() {
            panic!("Cannot get a node from a node that is not a part of a NodeTree!");
        }
        
        unsafe {
            Tp::new(self.tree.unwrap_unchecked(), self.rid, self.owner.unwrap_unchecked())
        }
    }

    /// Gets a `TpDyn` pointer to the owner of the node.
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
    ///
    /// # Note
    /// You can only have an owner on a node that is a part of a node tree.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn owner_dyn(&self) -> TpDyn {
        if self.tree().is_none() {
            panic!("Cannot get a node from a node that is not a part of a NodeTree!");
        }
        
        unsafe {
            TpDyn::new(self.tree.unwrap_unchecked(), self.rid, self.owner.unwrap_unchecked()).unwrap_unchecked()
        }
    }

    /// Sets the owner of the node.
    ///
    /// # Safety
    /// This should NOT be called manually.
    pub unsafe fn set_owner(&mut self, owner: RID) {
        self.owner = Some(owner);
    }

    /// Disconnects this node's owner from this node.
    ///
    /// # Safety
    /// This should NOT be called manually.
    pub unsafe fn disconnnect_owner(&mut self) {
        self.owner = None;
    }

    /// Gets a `Tp<T>` pointer to the direct parent of this node, if the node has one.
    /// Returns `Err` if there is no parent or if `T` does not match the parent's type.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn parent<T: Node>(&self) -> TreeResult<Tp<T>> {
        if self.tree().is_none() {
            panic!("Cannot get a node from a node that is not a part of a NodeTree!");
        }
        
        match self.parent {
            Some(parent) => {
                unsafe {
                    Tp::new(self.tree.unwrap_unchecked(), self.rid, parent)
                }
            },
            None => unsafe {
                TreeResult::new(self.tree.unwrap_unchecked(), self.rid, Err("This node has no parent".to_string()))
            }
        }
    }
    
    /// Gets a `TpDyn` pointer to the direct parent of this node, if the node has one.
    /// Returns `Err` if there is no parent.
    ///
    /// # Panics
    /// Panics if this Node is not connected to a `NodeTree`.
    pub fn parent_dyn(&self) -> TreeResult<TpDyn> {
        if self.tree().is_none() {
            panic!("Cannot get a node from a node that is not a part of a NodeTree!");
        }
        
        match self.parent {
            Some(parent) => {
                unsafe {
                    TpDyn::new(self.tree.unwrap_unchecked(), self.rid, parent)
                }
            },
            None => unsafe {
                TreeResult::new(self.tree.unwrap_unchecked(), self.rid, Err("This node has no parent".to_string()))
            }
        }
    }

    /// Sets the parent of this node.
    ///
    /// # Safety
    /// This should NOT be called manually.
    pub unsafe fn set_parent(&mut self, parent: RID) {
        self.parent = Some(parent);
    }

    /// Disconnects this node's parent from this node.
    ///
    /// # Safety
    /// This should NOT be called manually.
    pub unsafe fn disconnnect_parent(&mut self) {
        self.parent = None;
    }

    /// Gets the node's status.
    pub fn status(&self) -> NodeStatus {
        self.status.lock().unwrap().to_owned()
    }

    /// Sets the node's status.
    ///
    /// # Safety
    /// This should NOT be called manually.
    pub unsafe fn set_status(&self, status: NodeStatus) {
        *self.status.lock().unwrap() = status;
    }

    /// Gets the node's depth.
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Sets the node's depth.
    ///
    /// # Safety
    /// This should NOT be called manually.
    pub unsafe fn set_depth(&mut self, depth: usize) {
        self.depth = depth;
    }

    /// Returns if this node is a part of the node tree.
    /// If this is false, then it is expected behaviour that this node does not have an owner or
    /// parent.
    pub fn in_tree(&self) -> bool {
        self.tree().is_some()
    }

    /// Returns if this node is not a part of the node tree.
    /// If this is true, then it is expected behaviour that this node does not have an owner or
    /// parent.
    pub fn is_stray(&self) -> bool {
        self.tree().is_none()
    }

    /// Returns true if this node is the root node of the node tree.
    ///
    /// # Note
    /// This will return false if the node is not apart of a node tree.
    pub fn is_root(&self) -> bool {
        self.parent.is_none() && self.in_tree()
    }

    /// Returns true if this node is an owner of a scene.
    ///
    /// # Note
    /// This will return false if the node is not apart of a node tree.
    pub fn is_owner(&self) -> bool {
        match self.owner {
            Some(owner) => self.rid == owner,
            None        => false
        }
    }

    /// Returns the number of children this node has.
    pub fn num_children(&self) -> usize {
        self.children().len()
    }

    /// Returns true if this node has no children.
    pub fn childless(&self) -> bool {
        self.num_children() == 0
    }

    /// Marks this node as just having been recently loaded from the disk.
    /// 
    /// # Safety
    /// This should not be used manually.
    pub unsafe fn mark_as_loaded(&mut self) {
        self.loaded = true;
    }

    /// Marks this node as having been fully loaded.
    /// 
    /// # Safety
    /// This should not be used manually.
    pub unsafe fn mark_as_final(&mut self) {
        self.loaded = false;
    }

    /// Returns if this node has just been loaded from the disk.
    pub fn has_just_loaded(&self) -> bool {
        self.loaded
    }
}

impl std::fmt::Debug for NodeBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Inner[{}] {{ ", self.name))?;
        
        if let Some(parent) = &self.parent {
            f.write_str(&format!("Parent: {}, ", parent/*.name()*/))?;
        }
        if let Some(owner) = &self.owner {
            f.write_str(&format!("Owner: {}, ", owner/*.name()*/))?;
        }

        f.write_str(&format!("Connected to Tree: {}, ", self.tree.is_some()))?;
        f.write_str(&format!("Children: {:?}, ", &self.children))?;
        f.write_str(&format!("Depth: {} ", self.depth))?;
        f.write_str("}")?;

        Ok(())
    }
}

impl Clone for NodeBase {
    fn clone(&self) -> Self {
        Self::new(self.name.clone())
    }
}
