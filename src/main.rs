#![feature(get_mut_unchecked, arbitrary_self_types, unsize, dispatch_from_dyn, allocator_api, coerce_unsized)]

pub mod structs;
pub mod traits;
pub mod utils;

use prelude::*;

pub mod prelude {
    pub use std::rc::Rc;
    pub use crate::structs::{ high_pointer::Hp, node_base::NodeBase, node_path::NodePath, node_tree::NodeTree };
    pub use crate::traits::{ dynamic::Dynamic, node::{ Node, NodeAbstract, DynNode, private::NodeSealed } };
}


fn main() {
    
    // Create the tree.
    let root: Hp<NodeA>    = NodeA::new("Root".to_string());
    let tree: Hp<NodeTree> = NodeTree::new(root);

    // Begin operations on the tree.
    tree.start();
    tree.process();
}


#[derive(Debug, Clone)]
pub struct NodeA {
    base: Rc<NodeBase>
}

impl Dynamic for NodeA { fn to_any(&self) -> &dyn std::any::Any { self } }

impl NodeAbstract for NodeA {
    fn as_dyn(self: Hp<Self>) -> DynNode      { self }
    fn base(self: Hp<Self>)   -> Rc<NodeBase> { self.base.clone() }
}

impl NodeA {
    fn new(name: String) -> Hp<Self> {
        Hp::new(NodeA { base: NodeBase::new(name) })
    }
}

impl Node for NodeA {
    fn ready(self: Hp<Self>) -> () {
        if self.depth() < 3 {
            self.add_child(NodeA::new(format!("{}_Node", self.depth() + 1)));
            self.add_child(NodeA::new(format!("{}_Node", self.depth() + 1)));
            self.add_child(NodeA::new(format!("{}_Node", self.depth() + 1)));
        }
        //println!("{:#?}", self.children());
    }

    fn process(self: Hp<Self>, delta: f32) -> () {
        println!("{} | {}", self.name(), 1f32 / delta);
        if self.is_root() {
            println!("{:?}", self.get_node(NodePath::from_str("1_Node/2_Node1/3_Node2")).unwrap())
        }

        if self.children().is_empty() {
            self.root().unwrap().queue_termination();
        }
    }
}

