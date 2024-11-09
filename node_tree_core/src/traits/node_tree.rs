//===================================================================================================================================================================================//
//
//  /$$   /$$                 /$$                 /$$$$$$$$                                  /$$$$$$$$                 /$$   /$$             
// | $$$ | $$                | $$                |__  $$__/                                 |__  $$__/                |__/  | $$             
// | $$$$| $$  /$$$$$$   /$$$$$$$  /$$$$$$          | $$  /$$$$$$   /$$$$$$   /$$$$$$          | $$  /$$$$$$  /$$$$$$  /$$ /$$$$$$   /$$$$$$$
// | $$ $$ $$ /$$__  $$ /$$__  $$ /$$__  $$         | $$ /$$__  $$ /$$__  $$ /$$__  $$         | $$ /$$__  $$|____  $$| $$|_  $$_/  /$$_____/
// | $$  $$$$| $$  \ $$| $$  | $$| $$$$$$$$         | $$| $$  \__/| $$$$$$$$| $$$$$$$$         | $$| $$  \__/ /$$$$$$$| $$  | $$   |  $$$$$$ 
// | $$\  $$$| $$  | $$| $$  | $$| $$_____/         | $$| $$      | $$_____/| $$_____/         | $$| $$      /$$__  $$| $$  | $$ /$$\____  $$
// | $$ \  $$|  $$$$$$/|  $$$$$$$|  $$$$$$$         | $$| $$      |  $$$$$$$|  $$$$$$$         | $$| $$     |  $$$$$$$| $$  |  $$$$//$$$$$$$/
// |__/  \__/ \______/  \_______/ \_______/         |__/|__/       \_______/ \_______/         |__/|__/      \_______/|__/   \___/ |_______/ 
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Stores important traits required to create a fully-fledged `NodeTree` inherited type.
//!

use std::any::Any;
use std::ops::{ Deref, DerefMut };

use crate::structs::node_tree_base::NodeTreeBase;


/*
 * Node
 *      Tree
 */


/// Every application that wishes to take advantage of the `NodeTree` system must have its root
/// struct inherit from this.
///
/// # Example
/// Here is an example implementation:
/// ```rust
/// use node_tree::prelude::*;
///
/// #[derive(Debug, Tree)]
/// pub struct TreeSimple {
///     base: Option<NodeTreeBase>
/// }
/// 
/// impl TreeSimple {
///     pub fn new<I: Instanceable>(scene: I, verbosity: LoggerVerbosity) -> Box<Self> {
///         let mut tree: Box<TreeSimple> = Box::new(TreeSimple {
///             base: None
///         });
///         
///         initialize_base(&mut tree, scene, verbosity); // Not running this will cause undefined behaviour!
///         tree
///     }
/// }
/// ```
/// Note that the `Tree` should be initialized on the heap, preferably with a Box<T> pointer.
pub trait NodeTree: Deref<Target = NodeTreeBase> + DerefMut + Any {

    /// Sets the `NodeTreeBase` struct.
    ///
    /// # Safety
    /// This should NOT be used manually.
    unsafe fn set_base(&mut self, base: NodeTreeBase);
    
    /// Returns a reference to the `NodeTreeBase` object.
    fn base(&self) -> &NodeTreeBase;
    
    /// Returns a mutable reference to the `NodeTreeBase` object.
    fn base_mut(&mut self) -> &mut NodeTreeBase;
    
    /// Gets this as a dynamic `NodeTree` object.
    fn as_dyn(&self) -> &dyn NodeTree;
    
    /// Gets this as a mutable dynamic `NodeTree` object.
    fn as_dyn_mut(&mut self) -> &mut dyn NodeTree;

    /// Gets this as a raw pointer to a dynamic `NodeTree` object.
    fn as_dyn_raw(&self) -> *const dyn NodeTree;
    
    /// Gets this as a raw mutable pointer to a dynamic `NodeTree` object.
    fn as_dyn_raw_mut(&mut self) -> *mut dyn NodeTree;

    /// Converts this into an Any type.
    fn as_any(&self) -> &dyn Any;

    /// Converts this into a mutable Any type.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
