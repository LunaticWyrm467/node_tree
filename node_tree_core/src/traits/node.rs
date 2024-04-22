//===================================================================================================================================================================================//
//
//  /$$   /$$                 /$$                 /$$$$$$$$                 /$$   /$$             
// | $$$ | $$                | $$                |__  $$__/                |__/  | $$             
// | $$$$| $$  /$$$$$$   /$$$$$$$  /$$$$$$          | $$  /$$$$$$  /$$$$$$  /$$ /$$$$$$   /$$$$$$$
// | $$ $$ $$ /$$__  $$ /$$__  $$ /$$__  $$         | $$ /$$__  $$|____  $$| $$|_  $$_/  /$$_____/
// | $$  $$$$| $$  \ $$| $$  | $$| $$$$$$$$         | $$| $$  \__/ /$$$$$$$| $$  | $$   |  $$$$$$ 
// | $$\  $$$| $$  | $$| $$  | $$| $$_____/         | $$| $$      /$$__  $$| $$  | $$ /$$\____  $$
// | $$ \  $$|  $$$$$$/|  $$$$$$$|  $$$$$$$         | $$| $$     |  $$$$$$$| $$  |  $$$$//$$$$$$$/
// |__/  \__/ \______/  \_______/ \_______/         |__/|__/      \_______/|__/   \___/ |_______/ 
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Stores important traits required to create a fully-fledged `Node` type, such as `NodeAbstract`,
//! `private::Sealed`, and `Node`.
//!

use std::rc::Rc;

use crate::structs::{ high_pointer::Hp, node_base::NodeBase, node_tree::ProcessMode };
use super::dynamic::Dynamic;


/// This will be hidden the moment Rust implements Private/Sealed traits officially.
/// Do NOT implement this manually.
pub mod private {
    use std::rc::Rc;

    use crate::prelude::Log;
    use crate::structs::logger::SystemCall;
    use crate::structs::node_base::NodeStatus;
    use crate::structs::node_tree::NodeIdentity;
    use crate::structs::{ high_pointer::Hp, node_base::NodeBase, node_path::NodePath, node_tree::NodeTree, /*node_query::NodeQuery*/ };
    use crate::utils::functions::ensure_unique_name;
    use super::{ NodeAbstract, DynNode };

    
    /// Contains sealed methods that should not be overriden for the Node trait.
    pub trait NodeSealed: NodeAbstract {
        
        /// Gets the name of the node.
        /// Each name must be unique within the context of the parent's children vector.
        fn name(self: Hp<Self>) -> String {
            self.base().name().to_owned()
        }

        /// Sets the name of the node.
        /// This will fail if the name is not unique within the context of the parent's children
        /// vector.
        /// Returns false if the operation fails.
        fn set_name(self: Hp<Self>, name: &str) -> bool {
            if let Some(parent) = self.parent() {
                let mut is_unique: bool         = true;
                let     neighbors: Vec<DynNode> = parent.children().iter().map(|a| a.to_owned()).collect();

                for neighbor in neighbors {
                    let neighbor_name: String = neighbor.name().to_string();
                    let self_name:     String = self.name();
                    if  neighbor_name == self_name {
                        continue;
                    }
                    if &neighbor_name == name {
                        is_unique = false;
                        break;
                    }
                }

                if is_unique {
                    unsafe {
                        self.set_name_unchecked(name);
                    }
                }
                is_unique
            } else {
                unsafe {
                    self.set_name_unchecked(name);
                }
                true
            }
        }

        /// Sets the name of the node without checking if the name is unique.
        /// This should only be implemented, but not used manually.
        unsafe fn set_name_unchecked(self: Hp<Self>, name: &str) -> () {
            let mut base: Rc<NodeBase> = self.base();
            Rc::get_mut_unchecked(&mut base).set_name_unchecked(name);
        }
        
        /// Gets the unique ID of the node.
        /// Each unique ID is unique within the context of the entire NodeTree.
        fn unique_id(self: Hp<Self>) -> String {
            self.base().unique_id().to_string()
        }

        /// Sets the unique ID of the node.
        /// This should only be implemented, but not used manually.
        unsafe fn set_unique_id(self: Hp<Self>, unique_id: String) -> () {
            let mut base: Rc<NodeBase> = self.base();
            Rc::get_mut_unchecked(&mut base).set_unique_id(unique_id);
        }

        /// Gets the reference to the root NodeTree structure, which controls the entire tree.
        /// This will return None if the node is not connected to the NodeTree.
        fn root(self: Hp<Self>) -> Option<Hp<NodeTree>> {
            self.base().root()
        }

        /// Sets the reference to the root NodeTree structure.
        unsafe fn set_root(self: Hp<Self>, root: Hp<NodeTree>) -> () {
            let mut base: Rc<NodeBase> = self.base();
            Rc::get_mut_unchecked(&mut base).set_root(root);
        }

        /// Disconnects the NodeTree from this node.
        /// This should only be implemented, but not used manually.
        unsafe fn disconnnect_root(self: Hp<Self>) -> () {
            let mut base: Rc<NodeBase> = self.base();
            Rc::get_mut_unchecked(&mut base).disconnnect_root();
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
        fn owner(self: Hp<Self>) -> Option<DynNode> {
            self.base().owner()
        }

        /// Sets the owner of the node.
        /// This should only be implemented, but not used manually.
        unsafe fn set_owner(self: Hp<Self>, owner: DynNode) -> () {
            let mut base: Rc<NodeBase> = self.base();
            Rc::get_mut_unchecked(&mut base).set_owner(owner);
        }

        /// Disconnects this node's owner from this node.
        /// This should only be implemented, but not used manually.
        unsafe fn disconnnect_owner(self: Hp<Self>) -> () {
            let mut base: Rc<NodeBase> = self.base();
            Rc::get_mut_unchecked(&mut base).disconnnect_owner();
        }

        /// Gets the direct parent of this node.
        fn parent(self: Hp<Self>) -> Option<DynNode> {
            self.base().parent()
        }

        /// Sets the parent of this node.
        /// This should only be implemented, but not used manually.
        unsafe fn set_parent(self: Hp<Self>, parent: DynNode) -> () {
            let mut base: Rc<NodeBase> = self.base();
            Rc::get_mut_unchecked(&mut base).set_parent(parent);
        }

        /// Disconnects this node's parent from this node.
        /// This should only be implemented, but not used manually.
        unsafe fn disconnnect_parent(self: Hp<Self>) -> () {
            let mut base: Rc<NodeBase> = self.base();
            Rc::get_mut_unchecked(&mut base).disconnnect_parent();
        }

        /// Gets a vector of this node's children.
        fn children(self: Hp<Self>) -> Vec<DynNode> {
            self.base().children().to_owned()
        }

        /// Gets the node's depth in the `NodeTree`.
        fn depth(self: Hp<Self>) -> usize {
            self.base().depth()
        }

        /// Sets the node's depth in the `NodeTree`.
        /// This should only be implemented, but not used manually.
        unsafe fn set_depth(self: Hp<Self>, depth: usize) -> () {
            let mut base: Rc<NodeBase> = self.base();
            Rc::get_mut_unchecked(&mut base).set_depth(depth);
        }
   
        /// Gets the node's status.
        fn status(self: Hp<Self>) -> NodeStatus {
            self.base().status().clone()
        }

        /// Sets the node's status.
        /// This should only be implemented, but not used manually.
        unsafe fn set_status(self: Hp<Self>, status: NodeStatus) -> () {
            let mut base: Rc<NodeBase> = self.base();
            Rc::get_mut_unchecked(&mut base).set_status(status);
        }
        
        /// Returns true if this node is a stray node with no parent or owner.
        /// This means that the node is not connected to a tree nor is connected to any other node
        /// aside from any of its children.
        fn is_stray(self: Hp<Self>) -> bool {
            self.parent().is_none() && self.owner().is_none()
        }

        /// Returns true if this node is the root node.
        fn is_root(self: Hp<Self>) -> bool {
            self.parent().is_none() && self.in_tree()
        }

        /// Returns if this node is a part of the node tree.
        /// If this is false, then it is expected behaviour that this node does not have an owner.
        fn in_tree(self: Hp<Self>) -> bool {
            self.root().is_some()
        }

        /// Returns the number of children this node has.
        fn num_children(self: Hp<Self>) -> usize {
            self.children().len()
        }

        /// Returns true if this node has no children.
        fn childless(self: Hp<Self>) -> bool {
            self.num_children() == 0
        }

        /// Adds a child to the node, automatically renaming it if its name is not unique in the
        /// node's children vector.
        /// If this node is connected to the node tree, then `_ready()` will automatically be
        /// propogated throughout its ranks.
        fn add_child(self: Hp<Self>, node: DynNode) -> () {

            // Ensure that the child's name within the context of this node's children is unique.
            let names_of_children: Vec<String> = self.children().iter().map(|c| c.name().to_string()).collect();
            let node_name:         String      = node.name().to_string();

            // Add the child to this node's children and connect it to its parent and owner nodes,
            // as well as the root tree structure's reference.
            unsafe {
                node.set_name_unchecked(&ensure_unique_name(&node_name, names_of_children));
                node.set_unique_id(self.root().expect("Parent does not have a root reference set!").register_node());
                node.set_parent(self.as_dyn());
                node.set_root(self.root().expect("Parent does not have a root reference set!"));
                node.set_depth(self.depth() + 1);   // This is the only place where depth is updated.
                self.add_child_unchecked(node);
            }

            let child: DynNode = self.children()[self.num_children() - 1];
            unsafe {
                child.set_owner(self.owner().unwrap());   // For now, we just propagate the root as the owner for all nodes.
            }
            for node in child.bottom_up(true) {
                node.ready();
            }

            self.post_to_log(Log::Debug(&format!("Node \"{}\" added to the scene as the child of \"{}\"! Unique ID of \"{}\" generated!", child.name(), self.name(), child.unique_id())));
        }

        /// Adds a child without performing any checks nor running the `ready()` function in the
        /// new nodes.
        /// This should only be implemented, but not used manually.
        unsafe fn add_child_unchecked(self: Hp<Self>, node: DynNode) -> () {
            let mut base: Rc<NodeBase> = self.base();
            Rc::get_mut_unchecked(&mut base).children_mut().push(node);
        }

        /// Removes a child but it does not destroy it, disconnecting from its parent.
        /// Both the child and its children will be disconnected from the tree and their owners.
        /// This will return whether the child node was successfully removed or not.
        fn remove_child(self: Hp<Self>, name: &str) -> bool {
            let child: Option<(usize, DynNode)> = self.children().into_iter().enumerate().find(|(_, c)| c.name() == name);
            if child.is_none() {
                self.post_to_log(Log::Warn(&format!("Attempted to remove invalid node of name \"{}\" from node \"{}\"!", name, self.name())));
                return false;
            }

            let (child_idx, child): (usize, DynNode) = child.unwrap();
            unsafe {
                self.remove_child_unchecked(child_idx);
                child.disconnnect_parent();
                child.disconnnect_owner();
                child.disconnnect_root();
                match self.root() {
                    Some(root) => root.unregister_node(child.unique_id()),
                    None       => ()
                }
            }
            for node in child.top_down(false) {
                unsafe {
                    node.disconnnect_owner();
                    node.disconnnect_root();
                    match self.root() {
                        Some(root) => root.unregister_node(node.unique_id()),
                        None       => ()
                    }
                }
            }
            self.post_to_log(Log::Debug(&format!("Removed child node \"{}\" from parent node \"{}\"!", child.name(), self.name())));
            true 
        }

        /// Removes a child based on index without cleaning up references to that child,
        /// nor disconnecting any of the child's children nodes from the tree.
        /// This should only be implemented, but not used manually.
        unsafe fn remove_child_unchecked(self: Hp<Self>, idx: usize) -> () {
            let mut base: Rc<NodeBase> = self.base();
            Rc::get_mut_unchecked(&mut base).children_mut().remove(idx);
        }

        /// Returns a child at the given index.
        /// If there is no child at the given index, then the NodeQuery will be empty.
        fn get_child(self: Hp<Self>, i: usize) -> Option<DynNode> {
            if i >= self.num_children() {
                None
            } else {
                Some(self.children()[i])
            }
        }

        /// Gets a node given a NodePath that is respective to this node as the root.
        fn get_node(self: Hp<Self>, mut path: NodePath) -> Option<DynNode> {
            let next_node: Option<String> = path.pop_front();
            match next_node {
                Some(target) => {
                    for node in self.children() {
                        if node.name() == target {
                            return node.get_node(path);
                        }
                    }
                    None
                },
                None => Some(self.as_dyn())
            }
        }

        /// Produces a normal top-down order iteration of all of the nodes connected to this node.
        /// This is used to handle a lot of the scene tree behaviour.
        /// If 'contains_self' is true, then the list will contain this node as well.
        /// # Note
        /// Nodes that are at the beginning of the children vector will be prioritized.
        fn top_down(self: Hp<Self>, contains_self: bool) -> Vec<DynNode> {
            let mut iter: Vec<DynNode> = if contains_self { vec![self.as_dyn()] } else { Vec::new() };
            self.top_down_tail(&mut iter);
            iter
        }

        /// The tail end recursive function for the `top_down` method.
        fn top_down_tail(self: Hp<Self>, iter: &mut Vec<DynNode>) -> () {
            *iter = iter.iter().chain(self.children().iter()).map(|a| a.to_owned()).collect();
            for child in self.children() {
                *iter = iter.iter().chain(child.children().iter()).map(|a| a.to_owned()).collect();
            }
        }

        /// Produces a reverse bottom-up order iteration of all of the nodes connected to this node.
        /// This is typically used to initialize nodes or scenes of nodes.
        /// If 'contains_self' is true, then the list will contain this node as well.
        fn bottom_up(self: Hp<Self>, contains_self: bool) -> Vec<DynNode> {
            let mut iter:  Vec<DynNode> = Vec::new();
            let     layer: Vec<DynNode> = self.gather_deepest();
            
            self.bottom_up_tail(&mut iter, layer);
            if contains_self {
                iter.push(self.as_dyn())
            }
            iter
        }

        /// This gathers the deepest nodes in the tree.
        fn gather_deepest(self: Hp<Self>) -> Vec<DynNode> {
            let mut deepest_nodes: Vec<DynNode> = Vec::new();
            for node in self.children() {
                deepest_nodes.append(&mut node.gather_deepest());
            }
            deepest_nodes
        }

        /// The tail end recursive function for the `bottom_up` method.
        /// Due to how this functions, this function call doesn't actually call itself on different
        /// layers of the node tree, but it rather calls itself.
        fn bottom_up_tail(self: Hp<Self>, iter: &mut Vec<DynNode>, layer: Vec<DynNode>) -> () {
            
            // If the layer if empty, then return.
            if layer.is_empty() {
                return;
            }

            // Define a function to filter out duplicates.
            fn filter_duplicates(arr: Vec<DynNode>) -> Vec<DynNode> {
                let mut unique: Vec<DynNode> = Vec::new();
                for item in arr {
                    let mut is_unique: bool = true;
                    
                    for unique_item in &unique {
                        if item.name() == unique_item.name() {
                            is_unique = false;
                            break;
                        }
                    }

                    if is_unique {
                        unique.push(item);
                    }
                }
                unique
            }

            // We get the next layer by getting the node's parents and filtering out duplicates.
            let next_layer: Vec<DynNode> = filter_duplicates(layer.iter().map(|node| node.parent().unwrap()).collect());
            for node in layer {
                iter.push(node);
            }

            // If the next layer is only made up of one node and said node has the same name as this
            // node, then return.
            if next_layer.len() == 1 && next_layer[0].name() == self.name() {
                return;
            }

            self.bottom_up_tail(iter, next_layer);
        }

        /// Gets this Node's absolute NodePath to the root of the tree.
        fn get_absolute_path(self: Hp<Self>) -> NodePath {
            let mut path: String = String::new();
            self.get_absolute_path_tail(&mut path);
            NodePath::from_str(&path)
        }

        /// The recursive tail for the `get_absolute_path` function.
        fn get_absolute_path_tail(self: Hp<Self>, path: &mut String) {
            *path = self.name() + &(if path.is_empty() { String::new() } else { "/".to_string() + path });
            if !self.is_root() {
                self.parent().unwrap().get_absolute_path_tail(path);
            }
        }

        /// Attempts to post a log to the logger.
        /// If this node has a unique identifier accessible by name, then that will be used as the
        /// node's identifier in the log.
        /// # Panics
        /// Panics if this Node is not connected to a NodeTree.
        fn post_to_log(self: Hp<Self>, log: Log) -> () {
            unsafe {
                match &log {
                    Log::Warn(str)  => self.set_status(NodeStatus::JustWarned(str.to_string())),
                    Log::Panic(str) => self.set_status(NodeStatus::JustPanicked(str.to_string())),
                    _               => ()
                }
            }

            match self.root() {
                Some(root) => {
                    match root.get_node_identity(self.unique_id()) {
                        Some(NodeIdentity::NodePath) => {
                            let path: String = self.get_absolute_path().to_string();
                            root.post_to_log(SystemCall::NodePath(&path), log);
                        },
                        Some(NodeIdentity::UniqueName(name)) => {
                            root.post_to_log(SystemCall::Named(&name), log);
                        },
                        None => {
                            panic!("This node ({}) has no identity! Cannot post to log on an unregistered or invalid node!", self.name());
                        }
                    }
                },
                None => panic!("Cannot post to log on a disconnected node!")
            }
        }

        /// Destroys the Node, removing it from any connected parent or children.
        /// If this is the root node, then the destruction of this node will result in the program
        /// itself terminating.
        fn free(self: Hp<Self>) -> () {
            if let Some(parent) = self.parent() {
                parent.remove_child(&self.name());
            }

            if self.is_root() {
                self.root().unwrap().terminate();
            }

            for node in self.bottom_up(true) {
                unsafe {
                    Hp::destroy(node);
                }
            }
        }
    }
}
impl <T: NodeAbstract> private::NodeSealed for T {}


/// Denotes a dynamic node object type.
pub type DynNode = Hp<dyn Node>;


/// This implements of of the node's abstract behaviours.
/// This, along with `Node` must be implemented in order to create a new node.
pub trait NodeAbstract: Dynamic + std::fmt::Debug {
    
    /// Gets this as a dynamic Node object.
    fn as_dyn(self: Hp<Self>) -> DynNode;

    /// Returns a counted reference to the base Node object.
    fn base(self: Hp<Self>) -> Rc<NodeBase>;
}


/// This only holds the node's 'programmable' behaviours.
/// This must be implemented along with `NodeAbstract` to create a new node.
pub trait Node: NodeAbstract + private::NodeSealed {
    
    /// This function can be overridden to facilitate this node's starting behaviour.
    /// This only runs once after the scene that the node is a part of is fully initialized.
    fn ready(self: Hp<Self>) -> () {}

    /// This function can be overridden to facilitate behaviour that must update on a timely
    /// manner.
    /// This runs once per tick, and returns a delta value capturing the time between frames.
    fn process(self: Hp<Self>, _delta: f32) -> () {}

    /// This function can be overrriden to facilitate this node's terminal behaviour.
    /// It is run immeditately after this node is queued for destruction.
    fn terminal(self: Hp<Self>) -> () {}

    /// This returns the node's process mode, and entirely effects how the process() function
    /// behaves.
    /// By default, this returns `Inherit`.
    /// # Note
    /// Any node at the root of the scene tree with the `Inherit` property will by default inherit
    /// the `Pausable` process mode.
    fn process_mode(self: Hp<Self>) -> ProcessMode {
        ProcessMode::Inherit
    }
}
