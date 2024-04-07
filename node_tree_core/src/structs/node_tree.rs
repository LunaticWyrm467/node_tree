use std::time::{ Duration, Instant };

use crate::structs::high_pointer::Hp;
use crate::traits::node::DynNode;


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


/// Holds a tree of self-managing processes or nodes in a structure that allows for the creation of
/// large scale programs or games.
#[derive(Debug)]
pub struct NodeTree {
    root:   DynNode,
    status: TreeStatus
}

impl NodeTree {
    
    /// Creates a new NodeTree with the given root node.
    /// Due to the nature of the NodeTree, it must be wrapped in a Arc<Mutex<T>>.
    pub fn new(root: DynNode) -> Hp<Self> {
        
        // Create the base NodeTree.
        let node_tree: Hp<NodeTree> = Hp::new(NodeTree {
            root,
            status: TreeStatus::Idle
        });

        // Since this is the root node, it's 'owner' will be itself.
        // It will also have no parent.
        unsafe {
            node_tree.root.set_root(node_tree);
            node_tree.root.set_owner(node_tree.root);
        }

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
}
