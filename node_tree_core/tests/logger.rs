#![feature(arbitrary_self_types)]

use node_tree::structs::logger::{ Logger, SystemCall };
use node_tree::prelude::*;


#[test]
pub fn test_logger_bare() -> () {
    let mut logger: Logger = Logger::new(LoggerVerbosity::All);
            logger.post(SystemCall::NodePath("../Grandparent/Parent/NodeA"), Log::Info("System A Initialized!"), None);
            logger.post(SystemCall::NodePath("../Grandparent/Parent/NodeB"), Log::Warn("Some issue occurred! (Simulated Warning)"), None);
            logger.post(SystemCall::NodePath("../Grandparent/Parent/NodeC"), Log::Panic("Some crash occured! (Simulated Crash)"), None);
    
    assert_eq!(logger.to_str().split("\n").collect::<Vec<_>>().len(), 12);
}

#[test]
pub fn test_logger_tree() -> () {
    
    // Create the tree.
    let root: Hp<LoggerNode> = LoggerNode::new("Root".to_string());
    let tree: Hp<NodeTree>   = NodeTree::new(root, LoggerVerbosity::All);

    // Begin operations on the tree.
    tree.start();
    tree.process();
}

#[derive(Debug, Clone, NodeSys)]
pub struct LoggerNode {
    base: Rc<NodeBase>
}

impl LoggerNode {
    fn new(name: String) -> Hp<Self> {
        Hp::new(LoggerNode { base: NodeBase::new(name) })
    }
}

impl Node for LoggerNode {
    fn ready(self: Hp<Self>) -> () {
        if self.depth() < 3 {
            self.add_child(LoggerNode::new(format!("{}_Node", self.depth() + 1)));
            self.add_child(LoggerNode::new(format!("{}_Node", self.depth() + 1)));
            self.add_child(LoggerNode::new(format!("{}_Node", self.depth() + 1)));
        }
    }

    fn process(self: Hp<Self>, _delta: f32) -> () {
        if self.name() == "3_Node2" && self.parent().unwrap().parent().unwrap().name() == "1_Node" {
            self.post_to_log(Log::Warn("Simulating warning!"));
        }

        if self.name() == "3_Node2" && self.parent().unwrap().parent().unwrap().name() == "1_Node2"{
            self.post_to_log(Log::Panic("Simulating panic!"));
        }
    }
}
