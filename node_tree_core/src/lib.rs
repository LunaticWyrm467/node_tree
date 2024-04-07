#![feature(get_mut_unchecked, arbitrary_self_types, unsize, dispatch_from_dyn, allocator_api, coerce_unsized)]

pub mod structs;
pub mod traits;
pub mod utils;
pub mod prelude {
    pub use std::rc::Rc;
    pub use node_tree_derive::NodeSys;
    pub use crate::structs::{ high_pointer::Hp, node_base::NodeBase, node_path::NodePath, node_tree::NodeTree, node_query::NodeQuery };
    pub use crate::traits::{ dynamic::Dynamic, node::{ Node, NodeAbstract, DynNode, private::NodeSealed } };
}
