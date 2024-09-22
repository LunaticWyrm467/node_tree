//===================================================================================================================================================================================//
//
//  /$$$$$$                       /$$                                                       /$$       /$$          
// |_  $$_/                      | $$                                                      | $$      | $$          
//   | $$   /$$$$$$$   /$$$$$$$ /$$$$$$    /$$$$$$  /$$$$$$$   /$$$$$$$  /$$$$$$   /$$$$$$ | $$$$$$$ | $$  /$$$$$$ 
//   | $$  | $$__  $$ /$$_____/|_  $$_/   |____  $$| $$__  $$ /$$_____/ /$$__  $$ |____  $$| $$__  $$| $$ /$$__  $$
//   | $$  | $$  \ $$|  $$$$$$   | $$      /$$$$$$$| $$  \ $$| $$      | $$$$$$$$  /$$$$$$$| $$  \ $$| $$| $$$$$$$$
//   | $$  | $$  | $$ \____  $$  | $$ /$$ /$$__  $$| $$  | $$| $$      | $$_____/ /$$__  $$| $$  | $$| $$| $$_____/
//  /$$$$$$| $$  | $$ /$$$$$$$/  |  $$$$/|  $$$$$$$| $$  | $$|  $$$$$$$|  $$$$$$$|  $$$$$$$| $$$$$$$/| $$|  $$$$$$$
// |______/|__/  |__/|_______/    \___/   \_______/|__/  |__/ \_______/ \_______/ \_______/|_______/ |__/ \_______/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Provides the `Instanceable` trait which marks objects that can be represented in the `NodeTree`
//! as nodes.
//! 

use super::node::Node;


/// This marks any object that can be referenced in the `NodeTree` as either a node or a collection
/// of nodes.
pub trait Instanceable {
    
    /// Goes through and iterates through all of the nodes that are represented by this collection.
    /// The arguments passed through are the pointers to the parent (if there is one) and the node.
    fn iterate<F: FnMut(Option<*mut dyn Node>, *mut dyn Node)>(self, iterator: F);
}
