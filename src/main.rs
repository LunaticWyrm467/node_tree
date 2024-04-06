#![feature(get_mut_unchecked, arbitrary_self_types, unsize, dispatch_from_dyn, allocator_api, coerce_unsized)]

pub mod structs;
pub mod traits;
pub mod utils;

use std::rc::Rc;
use std::sync::{ Arc, Mutex };

use crate::structs::{ high_pointer::Hp, node_base::NodeBase, node_tree::NodeTree };
use crate::traits::{ dynamic::Dynamic, node::{ Node, NodeAbstract, DynNode, private::NodeSealed } };

pub type MutableArc<T> = Arc<Mutex<T>>;


fn main() {
    
    // Create the tree.
    let root: Hp<NodeA>    = NodeA::new("Root".to_string());
    let tree: Hp<NodeTree> = NodeTree::new(root);

    // Begin operations on the tree.
    tree.clone().start();
    tree.process();
}


#[derive(Debug, Clone)]
pub struct NodeA {
    base: Rc<NodeBase>
}

impl NodeA {
    fn new(name: String) -> Hp<Self> {
        Hp::new(NodeA { base: NodeBase::new(name) })
    }
}

impl Dynamic for NodeA { fn to_any(&self) -> &dyn std::any::Any { self } }

impl NodeAbstract for NodeA {
    fn as_dyn(self: Hp<Self>) -> DynNode      { self }
    fn base(self: Hp<Self>)   -> Rc<NodeBase> { self.base.clone() }
}

impl Node for NodeA {
    fn ready(self: Hp<Self>) -> () {
        if self.is_root() {
            self.add_child(NodeA::new("Node".to_string()));
            self.add_child(NodeA::new("Node".to_string()));
            self.add_child(NodeA::new("Node".to_string()));
        }
        println!("{:?}", self.children());
    }

    fn process(self: Hp<Self>, delta: f32) -> () {
        //println!("{} | {}", self.name(), delta);
    }
}

