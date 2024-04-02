use std::{ sync::{ Arc, Mutex, MutexGuard }, time::{ Duration, Instant } };

use crate::{ traits::node::{ DynNode, NodeMutex }, MutableArc };


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
    Terminated
}


/// Holds a tree of self-managing processes or nodes in a structure that allows for the creation of
/// large scale programs or games.
pub struct NodeTree {
    root:   DynNode,
    status: TreeStatus
}

impl NodeTree {
    
    /// Creates a new NodeTree with the given root node.
    /// Due to the nature of the NodeTree, it must be wrapped in a Arc<Mutex<T>>.
    pub fn new(root: DynNode) -> MutableArc<Self> {
        
        // Create the base NodeTree.
        let node_tree: MutableArc<NodeTree> = Arc::new(Mutex::new(NodeTree {
            root,
            status: TreeStatus::Idle
        }));


        // Since this is the root node, it's 'owner' will be itself.
        // It will also have no parent.
        let     tree_guard: MutexGuard<NodeTree> = node_tree.lock().unwrap();
        let mut root_guard: NodeMutex            = tree_guard.root.lock().unwrap();
        unsafe {
            root_guard.set_root(node_tree.clone());
            root_guard.set_owner(tree_guard.root.clone());
        }
        drop(root_guard);
        drop(tree_guard);

        node_tree
    }

    /// Runs the starting process behaviour -
    /// (any code under all initialized node's `ready()` functions).
    pub fn start(&mut self) -> () {
        for node in self.root.lock().unwrap().bottom_up() {
            node.lock().unwrap().ready();
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
    pub fn process(&mut self) -> () {
        let mut now: Instant = Instant::now();
        loop {

            // Calculate the delta time in between frames.
            let elapsed: Duration = now.elapsed();
            let delta:   f32      = elapsed.as_secs_f32();
            now = Instant::now();
            
            // Process the node tree recursively.
            self.process_tail(self.root.clone(), delta, ProcessMode::Pausable);
            
            // If the tree is queued for termination, then quit the program.
            if self.status == TreeStatus::Terminated {
                break;
            }
        }
    }

    /// Calls to this function results in the program terminting.
    /// This doesn't terminate the program itself, rather it just queues the program for
    /// self-termination.
    pub fn queue_termination(&mut self) -> () {
        self.status = TreeStatus::Terminated;
    }

    /// The recursive tail-end of the process function which traverses down the node tree.
    fn process_tail(&mut self, node: DynNode, delta: f32, inherited_process_mode: ProcessMode) -> () {
        
        // Determine the process mode.
        let mut node_guard:   NodeMutex   = node.lock().unwrap();
        let mut process_mode: ProcessMode = node_guard.process_mode();
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
                    ProcessMode::Always   => node_guard.process(delta),
                    ProcessMode::Pausable => node_guard.process(delta),
                    ProcessMode::Inverse  => ()
                }
            },

            TreeStatus::Paused => {
                match process_mode {
                    ProcessMode::Inherit  => panic!("Inherited process mode not set!"),
                    ProcessMode::Always   => node_guard.process(delta),
                    ProcessMode::Pausable => (),
                    ProcessMode::Inverse  => node_guard.process(delta)
                }
            }

            TreeStatus::Terminated => {
                node_guard.terminal();
            }
        }

        // Go through each of the children and process them, perpetuating the recursive cycle.
        for node in node_guard.children() {
            self.process_tail(node.to_owned(), delta, process_mode.clone());
        }
    }
}
