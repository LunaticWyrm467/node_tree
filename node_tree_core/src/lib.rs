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
//! ```rust, ignore
//! use node_tree::prelude::*;
//!
//! class! {
//!     
//!     /// Documentation and attributes are supported!
//!     pub declare NodeName extends UniqueTraitGroup1, UniqueTraitGroup2; // Will need to write a separate `impl` for each trait listed here.
//!     
//!     /// A signal can be connected to node functions and emitted.
//!     /// Safety is guaranteed via the scene tree.
//!     pub signal on_event(param_name: Type, ..);
//!
//!     /// Constants are supported.
//!     const SOME_CONST: &str = "Hello";
//!
//!     /// Fields can be defined like so, with default or without default values.
//!     let field_uninit:      u8;
//!     let field_initialized: String = "These are not constant expressions!".to_string();
//!
//!     // Fields can have special attributes, like so:
//!     default let field_default: u8; // Will automatically initialize to zero.
//!     unique  let field_unique: *mut c_void; // When cloned or serialized, this will safetly be initialized as a `None` value.
//!
//!     // Exportable fields will be saved and loaded from whenever a node scene is serialized.
//!     // Note: All exported types will need to implement the `Exportable` trait.
//!     export         let some_parameter: String;
//!     export default let some_parameter_default: bool;
//!
//!     // Hooks are any system functions that can be overridden.
//!     // This include the constructor `_init()`, `loaded()`, `ready()`, `process()`, `terminal()`, and `process_mode()`.
//!
//!     /// The constructor may only need to be implemented if there exists fields that do not have
//!     /// a default value.
//!     /// Note that this macro will automatically create a `new()` invokation, even without a
//!     /// predefined `_init()` hook. All attributes given to this hook will be transferred to the
//!     /// `new()` function.
//!     hk _init(starter_value: u8) {
//!         
//!         // Initialize a value by creating a variable with the same field name:
//!         let field_uninit: u8 = starter_value;
//!     }
//!
//!     /// Functions can be declared as per usual.
//!     fn foo(bar: Type) -> Type {
//!         todo!();
//!     }
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
        logger::{ LoggerVerbosity, Log },
        node_base::{ NodeError, NodeBase },
        node_field::{ Field, ExportableField, UniqueField, DefaultField },
        node_path::NodePath,
        node_tree_base::{ NodeTreeBase, TreeStatus, TreeProcess, ProcessMode, TerminationReason, initialize_base },
        tree_pointer::{ TPError, Tp, TpDyn },
        tree_option::TreeOption,
        tree_result::TreeResult,
        node_scene::NodeScene,
        rid::RID,
        signals::Signal
    };
    pub use crate::traits::{
        node::{ Node, NodeAbstract },
        exportable::{ Voidable, Exportable },
        registered::Registered,
        node_tree::NodeTree,
        instanceable::Instanceable
    };
    pub use crate::{ nodepath, debug, info, warn, error };
}

pub mod startup {
    pub use ctor::ctor;
}

pub use portable_intertrait as intertrait;