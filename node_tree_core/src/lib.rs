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
//! An extendable system made up of autonomous execution services known as nodes organized in a tree of processes. Inspired by Godot! 
//!
//! A simple node implementation will look like the following:
//! ```rust
//! use node_tree::prelude::*;
//!
//!
//! #[derive(Debug, Clone, Abstract, Register)] // Nodes require `Debug` and `Clone`.
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
//!     // feel free to implement `loaded()`, `ready()`, `process()`, `terminal()` and/or `process_mode()`
//!     // here.
//! }
//! ```
//!
//! Or with the `class!` macro, you could have an even simpler node declaration:
//! ```rust
//! use node_tree::prelude::*;
//!
//!
//! class! {
//!     pub dec NodeA;
//!
//!     // See the class! documentation for how you can implement custom fields, functions, and
//!     // hooks such as `ready()` or `process()`.
//!
//!     // NodeBase initialization and attribute/derive macros are implemented for you!
//! }
//! ```

#![allow(clippy::match_like_matches_macro, clippy::should_implement_trait, clippy::inherent_to_string, clippy::single_match)]

pub mod services;
pub mod structs;
pub mod traits;
pub mod utils;
pub mod trees;
pub mod prelude {
    //! Contains everything you'll need to create and handle Nodes and NodeTrees.
    //! You'll probably want to import all from this module.
    
    pub use node_tree_derive::{ Abstract, Register, Tree, scene, connect, class };
    pub use crate::structs::{
        cloneable_types::{ Doc, Eoc, Voc },
        logger::{ LoggerVerbosity, Log },
        node_base::NodeBase,
        node_path::NodePath,
        node_tree_base::{ NodeTreeBase, TreeStatus, TreeProcess, ProcessMode, TerminationReason, initialize_base },
        tree_pointer::{ Tp, TpDyn },
        tree_option::TreeOption,
        tree_result::TreeResult,
        node_scene::NodeScene,
        rid::RID,
        signals::Signal
    };
    pub use crate::traits::{
        node::{ Node, NodeAbstract },
        serializable::Serializable,
        registered::Registered,
        node_tree::NodeTree,
        instanceable::Instanceable
    };
}

pub use ctor;
