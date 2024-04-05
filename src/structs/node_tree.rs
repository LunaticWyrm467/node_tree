use std::{ rc::Rc, time::{ Duration, Instant } };

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
    pub fn new(root: DynNode) -> Rc<Self> {
        
        // Create the base NodeTree.
        let node_tree: Rc<NodeTree> = Rc::new(NodeTree {
            root,
            status: TreeStatus::Idle
        });

        // Since this is the root node, it's 'owner' will be itself.
        // It will also have no parent.
        unsafe {
            node_tree.clone().root.clone().set_root(node_tree.clone());
            node_tree.clone().root.clone().set_owner(node_tree.root.clone());
        }

        node_tree
    }

    /// Runs the starting process behaviour -
    /// (any code under all initialized node's `ready()` functions).
    pub fn start(self: Rc<Self>) -> () {
        for node in self.root.clone().bottom_up(true) {
            node.ready();
        }
        
        let mut mut_self: Rc<Self> = self.clone();
        unsafe { Rc::get_mut_unchecked(&mut mut_self).status = TreeStatus::Running; }
    }

    /// Runs the process behaviour of the Node Tree -
    /// (any code under all initialized node's `process()` functions).
    /// This function will run in a loop forever until the program terminates internally.
    /// 
    /// # Panics
    /// This function will panic if the start() function hasn't ran before this function was
    /// called.
    pub fn process(self: Rc<Self>) -> () {
        let mut now: Instant = Instant::now();
        loop {

            // Calculate the delta time in between frames.
            let elapsed: Duration = now.elapsed();
            let delta:   f32      = elapsed.as_secs_f32();
            now = Instant::now();
            
            // Process the node tree recursively.
            self.clone().process_tail(self.root.clone(), delta, ProcessMode::Pausable);
            
            // If the tree is queued for termination, then quit the program.
            if self.status == TreeStatus::Terminated {
                break;
            }
        }
    }

    /// Calls to this function results in the program terminting.
    /// This doesn't terminate the program itself, rather it just queues the program for
    /// self-termination.
    pub fn queue_termination(self: Rc<Self>) -> () {
        let mut mut_self: Rc<Self> = self.clone();
        unsafe { Rc::get_mut_unchecked(&mut mut_self).status = TreeStatus::Terminated; }
    }

    /// The recursive tail-end of the process function which traverses down the node tree.
    fn process_tail(self: Rc<Self>, node: DynNode, delta: f32, inherited_process_mode: ProcessMode) -> () {
        
        // Determine the process mode.
        let mut process_mode: ProcessMode = node.clone().process_mode();
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
                    ProcessMode::Always   => node.clone().process(delta),
                    ProcessMode::Pausable => node.clone().process(delta),
                    ProcessMode::Inverse  => ()
                }
            },

            TreeStatus::Paused => {
                match process_mode {
                    ProcessMode::Inherit  => panic!("Inherited process mode not set!"),
                    ProcessMode::Always   => node.clone().process(delta),
                    ProcessMode::Pausable => (),
                    ProcessMode::Inverse  => node.clone().process(delta)
                }
            }

            TreeStatus::Terminated => {
                node.clone().terminal();
            }
        }

        // Go through each of the children and process them, perpetuating the recursive cycle.
        for node in node.clone().children() {
            self.clone().process_tail(node.to_owned(), delta, process_mode.clone());
        }
    }
}
