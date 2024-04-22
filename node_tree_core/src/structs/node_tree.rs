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
//! ```rust, ignore
//! #![feature(arbitrary_self_types)]   // Required for now.
//! use node_tree::prelude::*;
//! 
//! fn main() -> () {
//!     
//!     // Create the tree.
//!     let root: DynNode      = todo!();   // Run your custom node type's constructor here. (DynNode is just Hp<dyn Node>) 
//!     let tree: Hp<NodeTree> = NodeTree::new(root, LoggerVerbosity::NoDebug);
//! 
//!     // Begin operations on the tree.
//!     tree.start();
//!     tree.process();   // This will run an indefinite loop until the program exits.
//! }
//! ```

use std::collections::HashMap;
use std::time::{ Duration, Instant };

use nanoid::nanoid;

use crate::traits::node::DynNode;
use super::high_pointer::Hp;
use super::logger::*;
use super::node_path::NodePath;


#[derive(Debug, Clone, PartialEq)]
pub enum ProcessMode {
    Inherit,
    Always,
    Pausable,
    Inverse,
}

#[derive(Debug, Clone, PartialEq)]
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


/// Holds a tree of self-managing processes or nodes in a structure that allows for the creation of
/// large scale programs or games.
#[derive(Debug)]
pub struct NodeTree {
    root:   DynNode,
    logger: Logger,
    cache:  HashMap<String, NodeIdentity>,   // Unique ID, System Name.
    status: TreeStatus
}

impl NodeTree {
    
    /// Creates a new NodeTree with the given root node.
    /// Due to the nature of the NodeTree, it must be wrapped in a Arc<Mutex<T>>.
    pub fn new(root: DynNode, logger_verbosity: LoggerVerbosity) -> Hp<Self> {
        
        // Create the base NodeTree.
        let node_tree: Hp<NodeTree> = Hp::new(NodeTree {
            root,
            logger: Logger::new(logger_verbosity),
            cache:  HashMap::new(),
            status: TreeStatus::Idle
        });

        // Since this is the root node, it's 'owner' will be itself.
        // It will also have no parent.
        unsafe {
            node_tree.root.set_unique_id(node_tree.register_node());
            node_tree.root.set_root(node_tree);
            node_tree.root.set_owner(node_tree.root);
        }

        node_tree.post_to_log(
            SystemCall::Named("NodeTree"),
            Log::Debug(&format!(
                    "Node \"{}\" added to the scene as the root of the NodeTree! Unique ID of \"{}\" generated!",
                    node_tree.root.name(), node_tree.root.unique_id()
        )));
        node_tree
    }

    /// Runs the starting process behaviour -
    /// (any code under all initialized node's `ready()` functions).
    pub fn start(mut self: Hp<Self>) -> () {
        for node in self.root.bottom_up(true) {
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
    pub fn process(self: Hp<Self>) -> () {
        let mut now: Instant = Instant::now();
        loop {

            // Calculate the delta time in between frames.
            let elapsed: Duration = now.elapsed();
            let delta:   f32      = elapsed.as_secs_f32();
            now = Instant::now();
            
            // Process the node tree recursively.
            self.process_tail(self.root, delta, ProcessMode::Pausable);
            
            // If the tree is queued for termination, then quit the program.
            if self.status == TreeStatus::QueuedTermination || self.status == TreeStatus::Terminated {
                break;
            }
        }
    }

    /// Calls to this function results in the program terminting.
    /// This doesn't terminate the program itself, rather it just queues the program for
    /// self-termination.
    pub fn queue_termination(mut self: Hp<Self>) -> () {
        self.status = TreeStatus::QueuedTermination;
    }

    /// Immediately terminates the program without running any termination behaviours.
    pub fn terminate(mut self: Hp<Self>) -> () {
        self.status = TreeStatus::Terminated;
    }

    /// The recursive tail-end of the process function which traverses down the node tree.
    fn process_tail(self: Hp<Self>, node: DynNode, delta: f32, inherited_process_mode: ProcessMode) -> () {
        
        // Determine the process mode.
        let mut process_mode: ProcessMode = node.process_mode();
        if process_mode == ProcessMode::Inherit {
            process_mode = inherited_process_mode;
        }
        
        // Depending on the tree's status and the node's process mode, abide by the processing
        // rules.
        match self.status {
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
        for node in node.children() {
            self.process_tail(node, delta, process_mode.clone());
            if self.status == TreeStatus::Terminated {
                break;
            }
        }
    }

    /// Registers the node to the tree and gives it a unique ID.
    /// No node reference actually has to be passed as the unique ID is used as identification.
    pub fn register_node(mut self: Hp<Self>) -> String {
        let mut id: String = nanoid!();
        while self.cache.contains_key(&id) {
            id = nanoid!();
        }
        self.cache.insert(id.clone(), NodeIdentity::NodePath);
        id
    }

    /// Unregisters a node from the tree.
    /// This should not be used manually.
    pub unsafe fn unregister_node(mut self: Hp<Self>, node_id: String) -> () {
        self.cache.remove(&node_id);
    }

    /// Modifies a node's registered path to accept a name - Useful for singletons.
    /// Returns None if the ID is invalid, or a boolean value that if true means that the name was
    /// set properly.
    pub fn allow_access_by_name(mut self: Hp<Self>, node_id: String, name: String) -> Option<bool> {
        if !self.cache.contains_key(&node_id) {
            return None;
        }

        if !self.cache.keys().into_iter().all(|x| x.to_owned() != name) {
            return Some(false);
        }
        *(self.cache.get_mut(&node_id).unwrap()) = NodeIdentity::UniqueName(name);
        Some(true)
    }

    /// Gets a node via an absolute path.
    pub fn get_node(self: Hp<Self>, mut absolute_path: NodePath) -> Option<DynNode> {
        if Some(self.root.name()) != absolute_path.pop_front() {
            return None;
        }
        self.root.get_node(absolute_path)
    }

    /// Gets the node's identity.
    /// The node's identity determines if the Node is accessible directly by name, or if it
    /// requires a NodePath to access.
    /// It also affects the logger's output.
    pub fn get_node_identity(self: Hp<Self>, node_id: String) -> Option<NodeIdentity> {
        match self.cache.get(&node_id) {
            Some(identity) => Some(identity.to_owned()),
            None           => None
        }
    }
    
    /// Sets the default crash header message.
    pub fn set_default_header_on_panic(mut self: Hp<Self>, msg: &str) -> () {
        self.logger.set_default_header_on_panic(msg);
    }
    
    /// Sets the default crash footer message.
    pub fn set_default_footer_on_panic(mut self: Hp<Self>, msg: &str) -> () {
        self.logger.set_default_footer_on_panic(msg);
    }

    /// Posts a new message to the log.
    pub fn post_to_log(mut self: Hp<Self>, system: SystemCall, log: Log) -> () {
        let self_other: Hp<NodeTree> = self.clone();
        self.logger.post(system, log, Some(self_other));
    }

    /// Gets the current log as a string.
    pub fn get_log(self: Hp<Self>) -> String {
        self.logger.to_str().to_string()
    }
}
