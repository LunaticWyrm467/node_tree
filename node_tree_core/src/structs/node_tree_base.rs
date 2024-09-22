//===================================================================================================================================================================================//
//
//  /$$   /$$                 /$$                 /$$$$$$$$                                  /$$$$$$$                               
// | $$$ | $$                | $$                |__  $$__/                                 | $$__  $$                              
// | $$$$| $$  /$$$$$$   /$$$$$$$  /$$$$$$          | $$  /$$$$$$   /$$$$$$   /$$$$$$       | $$  \ $$  /$$$$$$   /$$$$$$$  /$$$$$$ 
// | $$ $$ $$ /$$__  $$ /$$__  $$ /$$__  $$         | $$ /$$__  $$ /$$__  $$ /$$__  $$      | $$$$$$$  |____  $$ /$$_____/ /$$__  $$
// | $$  $$$$| $$  \ $$| $$  | $$| $$$$$$$$         | $$| $$  \__/| $$$$$$$$| $$$$$$$$      | $$__  $$  /$$$$$$$|  $$$$$$ | $$$$$$$$
// | $$\  $$$| $$  | $$| $$  | $$| $$_____/         | $$| $$      | $$_____/| $$_____/      | $$  \ $$ /$$__  $$ \____  $$| $$_____/
// | $$ \  $$|  $$$$$$/|  $$$$$$$|  $$$$$$$         | $$| $$      |  $$$$$$$|  $$$$$$$      | $$$$$$$/|  $$$$$$$ /$$$$$$$/|  $$$$$$$
// |__/  \__/ \______/  \_______/ \_______/         |__/|__/       \_______/ \_______/      |_______/  \_______/|_______/  \_______/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! The `NodeTreeBase` is the core of your program. It handles process frames, pausing, etc.
//!
//! # Example
//! To get one to work, you'll need to create a `NodeTree` wrapper struct for it. Here, we'll be
//! using the `TreeSimple` struct which is provided by this library. It is a simple implementation
//! which is useful for cases where you won't be using another framework.
//!
//! Simply start out by instantiate a root node - whichever node you deem necessary - and
//! feed it to the tree's constructor.
//! Finally, run the `start()` and `process()` functions in that order. 
//! ```rust,ignore
//! use node_tree::trees::tree_simple::TreeSimple;
//! use node_tree::prelude::*;
//! 
//! fn main() {
//!     
//!     // Create the tree.
//!     let root: YourNodeType    = todo!();   // Run your custom node type's constructor here. 
//!     let tree: Box<TreeSimple> = TreeSimple::new(root, LoggerVerbosity::NoDebug);
//! 
//!     // Begin operations on the tree.
//!     tree.start();
//!     loop {
//!        if tree.process().has_terminated() {
//!            break;
//!        }
//!    }
//! }
//! ```

use std::collections::{HashMap, HashSet};
use std::time::{ Duration, Instant };

use crate::traits::{ node::Node, node_tree::NodeTree, node_getter::NodeGetter, instanceable::Instanceable };
use super::logger::*;
use super::node_base::NodeStatus;
use super::rid::{ RID, RIDHolder };


/*
 * Node Tree
 *      Enums
 */


/// Determines how a Node handles its `process()` function.
/// You may wish to have some nodes be active always, be pausible, or only run when the program is
/// paused.
/// `Inherit` is for nodes whose behaviour is inherited from parent nodes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessMode {
    Inherit,
    Always,
    Pausable,
    Inverse,
}

/// Determines the tree's current behaviour.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TreeStatus {

    /// The tree of processes has not started yet. `start()` must be ran before the `process()`
    /// method!
    Idle,

    /// The tree is currently processing standard frame-based behaviours.
    Process(TreeProcess),

    /// The tree is waiting for the current frame of processes to finish before going into
    /// `Terminating` mode.
    QueuedTermination(TreeProcess),

    /// A single frame where each node calls its `terminal()` method.
    Terminating,

    /// The tree is no longer active and the program can be shut down.
    Terminated
}

impl TreeStatus {
    
    /// Determines if the tree is still active.
    /// E.g. not `Idle` or `Terminated`.
    pub fn is_active(&self) -> bool {
        match self {
            Self::Idle | Self::Terminated => false,
            _                             => true
        }
    }

    /// Determines if the tree has terminated.
    pub fn has_terminated(&self) -> bool {
        match self {
            Self::Terminated => true,
            _                => false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TreeProcess {
    Running,
    Paused
}

#[derive(Debug, Clone)]
pub enum NodeIdentity {
    UniqueName(String),
    NodePath
}

impl NodeIdentity {
    
    /// Checks whether a name doesn't match this identity.
    pub fn does_not_match(&self, name: &str) -> bool {
        match self {
            Self::UniqueName(this_name) => this_name != name,
            Self::NodePath              => false
        }
    }
}


/*
 * Node Tree
 *      Base
 */


/// Holds a tree of self-managing processes or nodes in a structure that allows for the creation of
/// large scale programs or games.
#[derive(Debug)]
pub struct NodeTreeBase {
    logger:     Logger,
    nodes:      RIDHolder<*mut dyn Node>,
    identity:   HashMap<RID, NodeIdentity>,
    singletons: HashMap<String, RID>,
    status:     TreeStatus,
    last_frame: Instant
}

impl NodeTreeBase {

    /// The RID for the root node.
    const ROOT_RID: RID = 0;

    /// Creates an empty `NodeTreeBase`, ready for initialization.
    unsafe fn new(logger_verbosity: LoggerVerbosity) -> Self {

        // Creates a new RID holder which stores all of the Nodes.
        let nodes: RIDHolder<*mut dyn Node> = RIDHolder::new();

        // Create the NodeTreeBase.
        let node_tree: NodeTreeBase = NodeTreeBase {
            logger:     Logger::new(logger_verbosity),
            nodes,
            identity:   HashMap::new(),
            singletons: HashMap::new(),
            status:     TreeStatus::Idle,
            last_frame: Instant::now()
        };
        
        node_tree
    }
    
    /// Creates a new NodeTreeBase with the pointer to the outer `NodeTree` struct and the given root node.
    ///
    /// # Note
    /// The `outer` struct MUST be allocated on the heap! Using `Box<T>` will work just fine, and
    /// you'll be able to get a raw pointer by dereferencing the `Box<T>` like so:
    /// ```rust
    /// let mut foo: Box<usize> = Box::new(0);
    /// let     bar: *mut usize = &mut *foo;
    /// ```
    ///
    /// # Safety
    /// This is marked as unsafe because it relies on a raw pointer being passed in.
    /// It is undefined behaviour if the outer struct is not allocated on the heap.
    /// ...
    unsafe fn initialize<I: Instanceable>(&mut self, outer: *mut dyn NodeTree, scene: I) {

        // Go through each node that needs to be instanced in the scene.
        scene.iterate(|parent, node| {
            if let Some(parent) = parent {
                let parent: &mut dyn Node = unsafe { &mut *parent };
                parent.add_child_from_ptr(node, false);
            } else {
                self.identity.insert(Self::ROOT_RID, NodeIdentity::NodePath);

                // Since this is the root node, it's 'owner' will be itself.
                // It will also have no parent.
                let root: &mut dyn Node = unsafe { &mut *node };
                unsafe {
                    root.set_rid(Self::ROOT_RID);
                    root.set_owner(Self::ROOT_RID);
                    root.set_tree(outer);
                }

                self.logger.post_manual(
                    SystemCall::Named("NodeTree".to_string()),
                    Log::Debug(&format!(
                            "Node \"{}\" added to the scene as the root of the NodeTree! Unique ID of \"{}\" generated!",
                            root.name(), root.rid()
                    )));
                self.nodes.push(node);
            }
        });
    }

    /// Runs the starting process behaviour -
    /// (any code under all initialized node's `ready()` functions).
    ///
    /// # Panics
    /// This will panic if the tree status is anything but `Idle` (you cannot start a tree twice).
    pub fn start(&mut self) {
        match self.status {
            TreeStatus::Idle => (),
            _                => panic!("Attempted to start() a NodeTree with a status of {:?}!", self.status)
        }

        for node in self.get_all_valid_nodes_mut(&self.root().bottom_up(true)) {
            node.ready();
        }
        self.status = TreeStatus::Process(TreeProcess::Running);
    }

    /// Runs the process behaviour of the Node Tree for a single frame -
    /// (any code under all initialized node's `process()` functions).
    /// This returns the `TreeStatus`
    /// 
    /// # Panics
    /// This function will panic if the start() function hasn't ran before this function was
    /// called.
    pub fn process(&mut self) -> TreeStatus {

        // Return early if the tree is no longer active.
        if !self.status.is_active() {
            return self.status;
        }

        // Calculate the delta time in between frames.
        let now:     Instant  = Instant::now();
        let elapsed: Duration = now.duration_since(self.last_frame);
        let delta:   f32      = elapsed.as_secs_f32();
        self.last_frame       = now;

        // Reset the prior frame's node statuses.
        for node in self.get_nodes_mut(&self.root().top_down(true)) {
            unsafe {
                node.unwrap_unchecked().set_status(NodeStatus::Normal);
            }
        }

        // Process the node tree recursively.
        self.process_tail(Self::ROOT_RID, delta, ProcessMode::Pausable);

        // Check the tree's status.
        match self.status {
            TreeStatus::QueuedTermination(_) => self.status = TreeStatus::Terminating,
            TreeStatus::Terminating          => self.status = TreeStatus::Terminated,
            _                                => ()
        }
        self.status
    }

    /// Gets a reference to the Root node.
    pub fn root(&self) -> &dyn Node {
        unsafe {
            &**self.nodes.retrieve(Self::ROOT_RID).unwrap_unchecked()
        }
    }
    
    /// Gets a mutable reference to the Root node.
    pub fn root_mut(&mut self) -> &mut dyn Node {
        unsafe {
            &mut **self.nodes.modify(Self::ROOT_RID).unwrap_unchecked()
        }
    }

    /// Gets a raw pointer to a node reference given an `RID`.
    /// Returns `None` if the `RID` is invalid.
    pub fn get_node_raw(&self, rid: RID) -> Option<*const dyn Node> {
        self.nodes.retrieve(rid).map(|node| *node as *const dyn Node)
    }

    /// Gets a reference to a node reference given an `RID`.
    /// Returns `None` if the `RID` is invalid.
    pub fn get_node(&self, rid: RID) -> Option<&dyn Node> {
        self.nodes.retrieve(rid).map(|node| unsafe { &**node })
    }

    /// Gets a vector of node references given the passed `RID`s.
    pub fn get_nodes(&self, rids: &[RID]) -> Vec<Option<&dyn Node>> {
        rids.into_iter()
            .map(|rid| self.nodes.retrieve(*rid).map(|node| unsafe { &**node })).collect::<Vec<_>>()
    }
    
    /// Gets a vector of node references given the passed `RID`s.
    /// All invalid RIDs are simply ignored.
    pub fn get_all_valid_nodes(&self, rids: &[RID]) -> Vec<&dyn Node> {
        rids.into_iter()
            .filter_map(|rid| self.nodes.retrieve(*rid).map(|node| unsafe { &**node })).collect::<Vec<_>>()
    }
    
    /// Gets a raw mutable pointer to a node reference given an `RID`.
    /// Returns `None` if the `RID` is invalid.
    pub fn get_node_mut_raw(&self, rid: RID) -> Option<*mut dyn Node> {
        self.nodes.retrieve(rid).map(|node| *node)
    }

    /// Gets a mutable reference to a node reference given an `RID`.
    /// Returns `None` if the `RID` is invalid.
    pub fn get_node_mut(&mut self, rid: RID) -> Option<&mut dyn Node> {
        self.nodes.modify(rid).map(|node| unsafe { &mut **node })
    }

    /// Gets a vector of mutable node references given the passed `RID`s.
    /// # Panics
    /// Panics if there are duplicate `RID`s in the passed in slice, as you cannot hold two or more
    /// mutable references to one Node.
    pub fn get_nodes_mut(&mut self, rids: &[RID]) -> Vec<Option<&mut dyn Node>> {
        if rids.len() != rids.into_iter().collect::<HashSet<_>>().len() {
            panic!("Duplicate RIDs found!");
        }

        rids.into_iter()
            .map(|rid| self.nodes.retrieve(*rid).map(|node| unsafe { &mut **node })).collect::<Vec<_>>()
    }
    
    /// Gets a vector of mutable node references given the passed `RID`s.
    /// All invalid RIDs are simply ignored.
    /// # Panics
    /// Panics if there are duplicate `RID`s in the passed in slice, as you cannot hold two or more
    /// mutable references to one Node.
    pub fn get_all_valid_nodes_mut(&mut self, rids: &[RID]) -> Vec<&mut dyn Node> {
        if rids.len() != rids.into_iter().collect::<HashSet<_>>().len() {
            panic!("Duplicate RIDs found!");
        }

        rids.into_iter()
            .filter_map(|rid| self.nodes.retrieve(*rid).map(|node| unsafe { &mut **node })).collect::<Vec<_>>()
    }

    /// Calls to this function results in the program terminating.
    /// This doesn't terminate the program itself, rather it just queues the program for
    /// self-termination.
    /// # Note
    /// This does nothing if termination has already been queued.
    pub fn queue_termination(&mut self) {
        match self.status {
            TreeStatus::Process(process) => self.status = TreeStatus::QueuedTermination(process),
            _                            => ()
        }
    }

    /// Immediately terminates the program without running any termination behaviours.
    pub fn terminate(&mut self) {
        self.status = TreeStatus::Terminated;
    }

    /// The recursive tail-end of the process function which traverses down the node tree.
    fn process_tail(&mut self, node_rid: RID, delta: f32, inherited_process_mode: ProcessMode) {
        let status: TreeStatus    = self.status;
        let node:   &mut dyn Node = self.get_node_mut(node_rid).unwrap();
        
        // Determine the process mode.
        let mut process_mode: ProcessMode = node.process_mode();
        if process_mode == ProcessMode::Inherit {
            process_mode = inherited_process_mode;
        }
        
        // Depending on the tree's status and the node's process mode, abide by the processing
        // rules.
        match status {
            TreeStatus::Idle => panic!("The function `start()` was not called before the program was ran!"),
            TreeStatus::Process(process) | TreeStatus::QueuedTermination(process) => {
                match process {
                    TreeProcess::Running => {
                        match process_mode {
                            ProcessMode::Inherit  => panic!("Inherited process mode not set!"),
                            ProcessMode::Always   => node.process(delta),
                            ProcessMode::Pausable => node.process(delta),
                            ProcessMode::Inverse  => ()
                        }
                    },

                    TreeProcess::Paused => {
                        match process_mode {
                            ProcessMode::Inherit  => panic!("Inherited process mode not set!"),
                            ProcessMode::Always   => node.process(delta),
                            ProcessMode::Pausable => (),
                            ProcessMode::Inverse  => node.process(delta)
                        }
                    }
                }
            }
            
            TreeStatus::Terminating => node.terminal(),
            TreeStatus::Terminated  => ()
        }

        // Go through each of the children and process them, perpetuating the recursive cycle.
        for child_node in node.children().into_iter().map(|c| c.rid()).collect::<Vec<_>>() {
            self.process_tail(child_node, delta, process_mode.clone());
            if self.status == TreeStatus::Terminated {
                break;
            }
        }
    }

    /// Registers the node to the tree and gives it a unique RID.
    /// This should not be used manually.
    ///
    /// # Safety
    /// Assumes that the pointer was created from a box like so:
    /// ```rust,ignore
    /// Box::into_raw(Box::new(node))
    /// ```
    pub unsafe fn register_node(&mut self, node: *mut dyn Node) -> RID {
        let rid: RID = self.nodes.push(node);
        self.identity.insert(rid, NodeIdentity::NodePath);
        rid
    }

    /// Unregisters a node from the tree, returning the Node as a `Box<T>` if it existed.
    /// This should not be used manually.
    ///
    /// # Note
    /// This does not check if the Node was a singleton and thus cannot be unregistered.
    pub unsafe fn unregister_node(&mut self, rid: RID) -> Option<Box<dyn Node>> {
        
        // Remove this node from the singletons map if it is on there.
        let mut singleton_name: Option<String> = None;
        for (name, singleton_rid) in &self.singletons {
            if *singleton_rid == rid {
                singleton_name = Some(name.to_string());
            }
        }

        if let Some(singleton_name) = singleton_name {
            self.singletons.remove(&singleton_name);
        }

        // TODO: Register a singleton name directly on the node as well to save performance.

        // Unregister this node from the tree.
        let node: Option<*mut dyn Node> = self.nodes.take(rid);
        self.identity.remove(&rid);
        node.map(|ptr| Box::from_raw(ptr))
    }
    
    /// Converts a Node into a singleton which means that a node is allowed access by name.
    ///
    /// # Note:
    /// Returns None if the RID is invalid, or a boolean value that if true means that the name was
    /// set properly.
    pub fn register_as_singleton(&mut self, rid: RID, name: String) -> Option<bool> {
        if !self.identity.contains_key(&rid) {
            return None;
        }

        if !self.identity.values().into_iter().all(|x| x.does_not_match(&name)) {
            return Some(false);
        }

        self.identity.insert(rid, NodeIdentity::UniqueName(name.clone()));
        self.singletons.insert(name, rid);
        Some(true)
    }

    /// Gets a node's RID via either an absolute path or a name if it is valid, or None if it is
    /// not.
    pub fn get_node_rid<P: NodeGetter>(&self, absolute_path: P) -> Option<RID> {
        absolute_path.get_from(self)
    }

    /// Gets the node's identity.
    /// The node's identity determines if the Node is accessible directly by name, or if it
    /// requires a NodePath to access.
    /// It also affects the logger's output.
    pub fn get_node_identity(&self, rid: RID) -> Option<NodeIdentity> {
        match self.identity.get(&rid) {
            Some(identity) => Some(identity.to_owned()),
            None           => None
        }
    }
    
    /// Sets the default crash header message.
    pub fn set_default_header_on_panic(&mut self, msg: &str) {
        self.logger.set_default_header_on_panic(msg);
    }
    
    /// Sets the default crash footer message.
    pub fn set_default_footer_on_panic(&mut self, msg: &str) {
        self.logger.set_default_footer_on_panic(msg);
    }

    /// Posts a new message to the log.
    pub fn post(&mut self, calling: RID, log: Log) {
        let ptr: *mut NodeTreeBase = self;
        unsafe {
            if self.logger.post(calling, log, ptr) {
                self.terminate();
            }
        }
    }

    /// Gets the current log as a string.
    pub fn get_log(&self) -> &str {
        self.logger.to_str()
    }
}


impl <'a> NodeGetter for &'a str {
    fn get_from(&self, tree: &NodeTreeBase) -> Option<RID> {
        self.to_string().get_from(tree)
    }
}

impl NodeGetter for String {
    fn get_from(&self, tree: &NodeTreeBase) -> Option<RID> {
        tree.singletons.get(self).copied()
    }
}


/// Initializes the base `NodeTreeBase` field in a `NodeTree` inherited object.
///
/// # Safety
/// It is UNDEFINED behaviour to NOT call this function within a tree implementation's constructor.
pub fn initialize_base<T: NodeTree, I: Instanceable>(tree: &mut Box<T>, scene: I, verbosity: LoggerVerbosity) {
    let base: NodeTreeBase = unsafe { NodeTreeBase::new(verbosity) };
    unsafe {
        tree.set_base(base);

        let tree_ptr: *mut dyn NodeTree = tree.as_dyn_raw_mut();
        tree.base_mut().initialize(tree_ptr, scene);
    }
}
