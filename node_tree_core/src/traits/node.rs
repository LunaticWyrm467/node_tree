//===================================================================================================================================================================================//
//
//  /$$   /$$                 /$$                 /$$$$$$$$                 /$$   /$$             
// | $$$ | $$                | $$                |__  $$__/                |__/  | $$             
// | $$$$| $$  /$$$$$$   /$$$$$$$  /$$$$$$          | $$  /$$$$$$  /$$$$$$  /$$ /$$$$$$   /$$$$$$$
// | $$ $$ $$ /$$__  $$ /$$__  $$ /$$__  $$         | $$ /$$__  $$|____  $$| $$|_  $$_/  /$$_____/
// | $$  $$$$| $$  \ $$| $$  | $$| $$$$$$$$         | $$| $$  \__/ /$$$$$$$| $$  | $$   |  $$$$$$ 
// | $$\  $$$| $$  | $$| $$  | $$| $$_____/         | $$| $$      /$$__  $$| $$  | $$ /$$\____  $$
// | $$ \  $$|  $$$$$$/|  $$$$$$$|  $$$$$$$         | $$| $$     |  $$$$$$$| $$  |  $$$$//$$$$$$$/
// |__/  \__/ \______/  \_______/ \_______/         |__/|__/      \_______/|__/   \___/ |_______/ 
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Stores important traits required to create a fully-fledged `Node` type, such as `NodeAbstract`and `Node`.
//!
//! Most of the time, you'lll wish to make use of the `Abstract` derive macro, which implements all
//! but the `Node` trait for you! You'll require the `Debug` and `Clone` trait to be derived as
//! well, however.
//!

use std::fmt;
use std::any::Any;
use std::ops::{ Deref, DerefMut };

use crate::structs::{ node_base::NodeBase, node_tree_base::ProcessMode };
use super::instanceable::Instanceable;


/// This implements of of the node's abstract behaviours.
/// This, along with `Node` must be implemented in order to create a new node.
pub trait NodeAbstract: Deref<Target = NodeBase> + DerefMut + Any + fmt::Debug {
    
    /// Returns a reference to the `NodeBase` object.
    fn base(&self) -> &NodeBase;
    
    /// Returns a mutable reference to the `NodeBase` object.
    fn base_mut(&mut self) -> &mut NodeBase;
    
    /// Gets this as a dynamic Node object.
    fn as_dyn(&self) -> &dyn Node;
    
    /// Gets this as a mutable dynamic Node object.
    fn as_dyn_mut(&mut self) -> &mut dyn Node;

    /// Gets this as a raw pointer to a dynamic Node object.
    fn as_dyn_raw(&self) -> *const dyn Node;

    /// Gets this as a raw mutable pointer to a dynamic Node object.
    fn as_dyn_raw_mut(&mut self) -> *mut dyn Node;

    /// Converts this into a Boxed type.
    fn to_dyn_box(self) -> Box<dyn Node>;

    /// Converts this into an Any type.
    fn as_any(&self) -> &dyn Any;

    /// Converts this into a mutable Any type.
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Clones this, returning a boxed clone value.
    fn clone_as_instance(&self) -> Box<dyn Node>;
}


/// This only holds the node's 'programmable' behaviours.
/// This must be implemented along with `NodeAbstract` to create a new node.
pub trait Node: NodeAbstract {
    
    /// This function can be overridden to facilitate this node's starting behaviour.
    /// This only runs once after the scene that the node is a part of is fully initialized.
    fn ready(&mut self) -> () {}

    /// This function can be overridden to facilitate behaviour that must update on a timely
    /// manner.
    /// This runs once per tick, and returns a delta value capturing the time between frames.
    fn process(&mut self, _delta: f32) -> () {}

    /// This function can be overrriden to facilitate this node's terminal behaviour.
    /// It is run immeditately after this node is queued for destruction.
    fn terminal(&mut self) -> () {}

    /// This returns the node's process mode, and entirely effects how the process() function
    /// behaves.
    /// By default, this returns `Inherit`.
    /// # Note
    /// Any node at the root of the scene tree with the `Inherit` property will by default inherit
    /// the `Pausable` process mode.
    fn process_mode(&self) -> ProcessMode {
        ProcessMode::Inherit
    }
}

impl <N: Node> Instanceable for N {
    fn iterate<F: FnMut(Option<*mut dyn Node>, *mut dyn Node)>(self, mut iterator: F) {
        iterator(None, Box::into_raw(self.to_dyn_box()));
    }
}
