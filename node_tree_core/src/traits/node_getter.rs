//===================================================================================================================================================================================//
//
//  /$$   /$$                 /$$                  /$$$$$$              /$$     /$$                        
// | $$$ | $$                | $$                 /$$__  $$            | $$    | $$                        
// | $$$$| $$  /$$$$$$   /$$$$$$$  /$$$$$$       | $$  \__/  /$$$$$$  /$$$$$$ /$$$$$$    /$$$$$$   /$$$$$$ 
// | $$ $$ $$ /$$__  $$ /$$__  $$ /$$__  $$      | $$ /$$$$ /$$__  $$|_  $$_/|_  $$_/   /$$__  $$ /$$__  $$
// | $$  $$$$| $$  \ $$| $$  | $$| $$$$$$$$      | $$|_  $$| $$$$$$$$  | $$    | $$    | $$$$$$$$| $$  \__/
// | $$\  $$$| $$  | $$| $$  | $$| $$_____/      | $$  \ $$| $$_____/  | $$ /$$| $$ /$$| $$_____/| $$      
// | $$ \  $$|  $$$$$$/|  $$$$$$$|  $$$$$$$      |  $$$$$$/|  $$$$$$$  |  $$$$/|  $$$$/|  $$$$$$$| $$      
// |__/  \__/ \______/  \_______/ \_______/       \______/  \_______/   \___/   \___/   \_______/|__/      
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Provides the `NodeGetter` trait which is implemented for types that can be interpreted as node
//! addresses in the tree.
//! 

use crate::structs::{ node_tree_base::NodeTreeBase, rid::RID };


/// A trait that is implemented for types that can be used to get node RIDs from the `NodeTree`.
pub trait NodeGetter {
    
    /// A function that must be implemented per compatible type.
    fn get_from(&self, tree: &NodeTreeBase) -> Option<RID>;
}
