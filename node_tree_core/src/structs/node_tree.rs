//===================================================================================================================================================================================//
//
//  /$$   /$$                 /$$                 /$$$$$$$$                           
// | $$$ | $$                | $$                |__  $$__/                           
// | $$$$| $$  /$$$$$$   /$$$$$$$  /$$$$$$          | $$  /$$$$$$   /$$$$$$   /$$$$$$ 
// | $$ $$ $$ /$$__  $$ /$$__  $$ /$$__  $$         | $$ /$$__  $$ /$$__  $$ /$$__  $$
// | $$  $$$$| $$  \ $$| $$  | $$| $$$$$$$$         | $$| $$  \__/| $$$$$$$$| $$$$$$$$
// | $$\  $$$| $$  | $$| $$  | $$| $$_____/         | $$| $$      | $$_____/| $$_____/
// | $$ \  $$|  $$$$$$/|  $$$$$$$|  $$$$$$$         | $$| $$      |  $$$$$$$|  $$$$$$$
// |__/  \__/ \______/  \_______/ \_______/         |__/|__/       \_______/ \_______/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! The `NodeTree` is the core of your program. It handles process frames, pausing, etc.
//!
//! # Example
//! To get one to work, simply instantiate a root node - whichever node you deem necessary - and
//! feed it to the tree's constructor.
//! Finally, run the `start()` and `process()` functions in that order. 
//! ```rust,ignore
//! #![feature(arbitrary_self_types)]   // Required for now.
//! use node_tree::prelude::*;
//! 
//! fn main() -> () {
//!     
//!     // Create the tree.
//!     let root: YourNodeType  = todo!();   // Run your custom node type's constructor here. 
//!     let tree: Box<NodeTree> = NodeTree::new(root, LoggerVerbosity::NoDebug);
//! 
//!     // Begin operations on the tree.
//!     tree.start();
//!     tree.process();   // This will run an indefinite loop until the program exits.
//! }
//! ```

use std::collections::{HashMap, HashSet};
use std::time::{ Duration, Instant };

use crate::traits::{ node::Node, node_getter::NodeGetter };
use super::logger::*;
use super::node_base::NodeStatus;
use super::rid::{ RID, RIDHolder };


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessMode {
    Inherit,
    Always,
    Pausable,
    Inverse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TreeStatus {
    Idle,
    Running,
    Paused,
    QueuedTermination,
    Terminated
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


/// Holds a tree of self-managing processes or nodes in a structure that allows for the creation of
/// large scale programs or games.
#[derive(Debug)]
pub struct NodeTree {
    logger:     Logger,
    nodes:      RIDHolder<*mut dyn Node>,
    identity:   HashMap<RID, NodeIdentity>,
    singletons: HashMap<String, RID>,
    status:     TreeStatus
}

impl NodeTree {

    /// The RID for the root node.
    const ROOT_RID: RID = 0;
    
    /// Creates a new NodeTree with the given root node.
    pub fn new<N: Node>(root: N, logger_verbosity: LoggerVerbosity) -> Box<Self> {

        // Creates a new RID holder which stores all of the Nodes.
        let mut nodes: RIDHolder<*mut dyn Node> = RIDHolder::new();
        let     _:     RID                      = nodes.push(Box::into_raw(root.to_dyn_box())); // This will ALWAYS be zero!

        // Create the base NodeTree.
        let mut node_tree: Box<NodeTree> = Box::new(NodeTree {
            logger:     Logger::new(logger_verbosity),
            nodes,
            identity:   HashMap::new(),
            singletons: HashMap::new(),
            status:     TreeStatus::Idle
        });
        let tree: *mut NodeTree = &mut *node_tree;

        // Since this is the root node, it's 'owner' will be itself.
        // It will also have no parent.
        unsafe {
            let root: &mut dyn Node = node_tree.root_mut();

            root.set_rid(Self::ROOT_RID);
            root.set_owner(Self::ROOT_RID);
            root.set_tree(tree);
        }

        let root: &dyn Node = node_tree.root();
        node_tree.logger.post_manual(
            SystemCall::Named("NodeTree".to_string()),
            Log::Debug(&format!(
                    "Node \"{}\" added to the scene as the root of the NodeTree! Unique ID of \"{}\" generated!",
                    root.name(), root.rid()
        )));
        node_tree
    }

    /// Runs the starting process behaviour -
    /// (any code under all initialized node's `ready()` functions).
    pub fn start(&mut self) {
        for node in self.get_all_valid_nodes_mut(&self.root().bottom_up(true)) {
            node.ready();
        }
        
        self.status = TreeStatus::Running;
    }

    /// Runs the process behaviour of the Node Tree -
    /// (any code under all initialized node's `process()` functions).
    /// This function will run in a loop forever until the program terminates internally.
    /// 
    /// # Panics
    /// This function will panic if the start() function hasn't ran before this function was
    /// called.
    pub fn process(&mut self) {
        let mut now: Instant = Instant::now();
        loop {

            // Calculate the delta time in between frames.
            let elapsed: Duration = now.elapsed();
            let delta:   f32      = elapsed.as_secs_f32();
            now = Instant::now();

            // Reset the prior frame's node statuses.
            for node in self.get_nodes_mut(&self.root().top_down(true)) {
                unsafe {
                    node.unwrap_unchecked().set_status(NodeStatus::Normal);
                }
            }
            
            // Process the node tree recursively.
            self.process_tail(Self::ROOT_RID, delta, ProcessMode::Pausable);
            
            // If the tree is queued for termination, then quit the program.
            if self.status == TreeStatus::QueuedTermination || self.status == TreeStatus::Terminated {
                break;
            }
        }
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
    pub fn queue_termination(&mut self) {
        self.status = TreeStatus::QueuedTermination;
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

            TreeStatus::Running => {
                match process_mode {
                    ProcessMode::Inherit  => panic!("Inherited process mode not set!"),
                    ProcessMode::Always   => node.process(delta),
                    ProcessMode::Pausable => node.process(delta),
                    ProcessMode::Inverse  => ()
                }
            },

            TreeStatus::Paused => {
                match process_mode {
                    ProcessMode::Inherit  => panic!("Inherited process mode not set!"),
                    ProcessMode::Always   => node.process(delta),
                    ProcessMode::Pausable => (),
                    ProcessMode::Inverse  => node.process(delta)
                }
            }

            TreeStatus::QueuedTermination => node.terminal(),
            TreeStatus::Terminated        => ()
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
    pub unsafe fn register_node(&mut self, node: Box<dyn Node>) -> RID {
        let rid: RID = self.nodes.push(Box::into_raw(node));
        self.identity.insert(rid, NodeIdentity::NodePath);
        rid
    }

    /// Unregisters a node from the tree, returning the Node if it existed.
    /// This should not be used manually.
    /// # Note
    /// This does not check if the Node was a singleton and thus cannot be unregistered.
    pub unsafe fn unregister_node(&mut self, rid: RID) -> Option<Box<dyn Node>> {
        let node: Option<*mut dyn Node> = self.nodes.take(rid);
        self.identity.remove(&rid);
        node.map(|ptr| Box::from_raw(ptr))
    }
    
    /// Converts a Node into a singleton which means that a node is allowed access by name.
    /// # Note:
    /// Singleton nodes cannot be destroyed or detached from the scene tree.
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
        let ptr: *mut NodeTree = self;
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
    fn get_from(&self, tree: &NodeTree) -> Option<RID> {
        self.to_string().get_from(tree)
    }
}

impl NodeGetter for String {
    fn get_from(&self, tree: &NodeTree) -> Option<RID> {
        tree.singletons.get(self).copied()
    }
}
