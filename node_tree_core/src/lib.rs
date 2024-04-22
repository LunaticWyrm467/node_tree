//===================================================================================================================================================================================//
//
//  /$$$$$$$                        /$$    
// | $$__  $$                      | $$    
// | $$  \ $$  /$$$$$$   /$$$$$$  /$$$$$$  
// | $$$$$$$/ /$$__  $$ /$$__  $$|_  $$_/  
// | $$__  $$| $$  \ $$| $$  \ $$  | $$    
// | $$  \ $$| $$  | $$| $$  | $$  | $$ /$$
// | $$  | $$|  $$$$$$/|  $$$$$$/  |  $$$$/
// |__/  |__/ \______/  \______/    \___/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! The root file of the library.
//! Contains the `prelude` module which you will probably want to import from.
//!
//! A simple node implementation will look like the following:
//! ```
//! #![feature(arbitrary_self_types)]   // Required for now.
//! use node_tree::prelude::*;
//!
//!
//! #[derive(Debug, Clone, NodeSys)]
//! pub struct NodeA {
//!     base: Rc<NodeBase>   // Required for Nodes.
//! }
//! 
//! // To make things simple, it is advised to have most node constructors return the node
//! // instance wrapped inside of this crate's `Hp<T>` pointer.
//! impl NodeA {
//!     fn new(name: String) -> Hp<Self> {
//!         Hp::new(NodeA { base: NodeBase::new(name) })
//!     }
//! }
//!
//! impl Node for NodeA {
//!     // feel free to implement `ready()`, `process()`, `terminal()` and/or `process_mode()`
//!     // here.
//! }
//! ```

#![feature(get_mut_unchecked, arbitrary_self_types, unsize, dispatch_from_dyn, allocator_api, coerce_unsized)]

pub mod structs;
pub mod traits;
pub mod utils;
pub mod prelude {
    //! Contains everything you'll need to create and handle Nodes and NodeTrees.
    //! You'll probably want to import all from this module.

    pub use std::rc::Rc;
    pub use node_tree_derive::NodeSys;
    pub use crate::structs::{ high_pointer::Hp, logger::{ LoggerVerbosity, Log }, node_base::NodeBase, node_path::NodePath, node_tree::NodeTree, node_query::NodeQuery };
    pub use crate::traits::{ dynamic::Dynamic, node::{ Node, NodeAbstract, DynNode, private::NodeSealed } };
}
