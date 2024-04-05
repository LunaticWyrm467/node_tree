#![feature(get_mut_unchecked)]

pub mod structs;
pub mod traits;
pub mod utils;

use std::rc::Rc;
use std::sync::{ Arc, Mutex };

use crate::structs::{ node_base::NodeBase, node_tree::NodeTree };
use crate::traits::{ dynamic::Dynamic, node::{ Node, NodeAbstract, DynNode, private::NodeSealed } };

pub type MutableArc<T> = Arc<Mutex<T>>;


fn main() {
    
    // Create the tree.
    let root: Rc<NodeA>    = NodeA::new("Root".to_string());
    let tree: Rc<NodeTree> = NodeTree::new(root);

    // Begin operations on the tree.
    tree.clone().start();
    tree.process();
}


#[derive(Debug)]
pub struct NodeA {
    base: Rc<NodeBase>
}

impl NodeA {
    fn new(name: String) -> Rc<Self> {
        Rc::new(NodeA { base: NodeBase::new(name) })
    }
}

impl Dynamic for NodeA { fn to_any(&self) -> &dyn std::any::Any { self } }

impl NodeAbstract for NodeA {
    fn as_dyn(self: Rc<Self>) -> DynNode      { self }
    fn base(self: Rc<Self>)   -> Rc<NodeBase> { self.base.clone() }
}

impl Node for NodeA {
    fn ready(self: Rc<Self>) -> () {
        if self.clone().is_root() {
            self.clone().add_child(NodeA::new("Node".to_string()));
            self.clone().add_child(NodeA::new("Node".to_string()));
            self.clone().add_child(NodeA::new("Node".to_string()));
        }
    }

    fn process(self: Rc<Self>, delta: f32) -> () {
        println!("{} | {}", self.name(), delta);
    }
}
