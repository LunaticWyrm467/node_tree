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
//! ```rust
//! use node_tree::prelude::*;
//!
//!
//! #[derive(Debug, Clone, Abstract)] // Nodes require `Debug` and `Clone`.
//! pub struct NodeA {
//!     base: NodeBase   // Required for Nodes.
//! }
//! 
//! impl NodeA {
//!     fn new(name: String) -> Self {
//!         NodeA { base: NodeBase::new(name) }
//!     }
//! }
//!
//! impl Node for NodeA {
//!     // feel free to implement `ready()`, `process()`, `terminal()` and/or `process_mode()`
//!     // here.
//! }
//! ```

pub mod structs;
pub mod traits;
pub mod utils;
pub mod trees;
pub mod prelude {
    //! Contains everything you'll need to create and handle Nodes and NodeTrees.
    //! You'll probably want to import all from this module.

    pub use node_tree_derive::{ Abstract, Tree, scene };
    pub use crate::structs::{
        cloneable_types::{ Doc, Eoc, Voc },
        logger::{ LoggerVerbosity, Log },
        node_base::NodeBase,
        node_path::NodePath,
        node_tree_base::{ NodeTreeBase, TreeStatus, TreeProcess, ProcessMode, initialize_base },
        tree_pointer::{ Tp, TpDyn },
        node_scene::NodeScene,
        rid::RID,
    };
    pub use crate::traits::{
        node::{ Node, NodeAbstract },
        node_tree::NodeTree,
        instanceable::Instanceable
    };
}
