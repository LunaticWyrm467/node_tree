//===================================================================================================================================================================================//
//
//  /$$$$$$$                      /$$             /$$                                         /$$
// | $$__  $$                    |__/            | $$                                        | $$
// | $$  \ $$  /$$$$$$   /$$$$$$  /$$  /$$$$$$$ /$$$$$$    /$$$$$$   /$$$$$$   /$$$$$$   /$$$$$$$
// | $$$$$$$/ /$$__  $$ /$$__  $$| $$ /$$_____/|_  $$_/   /$$__  $$ /$$__  $$ /$$__  $$ /$$__  $$
// | $$__  $$| $$$$$$$$| $$  \ $$| $$|  $$$$$$   | $$    | $$$$$$$$| $$  \__/| $$$$$$$$| $$  | $$
// | $$  \ $$| $$_____/| $$  | $$| $$ \____  $$  | $$ /$$| $$_____/| $$      | $$_____/| $$  | $$
// | $$  | $$|  $$$$$$$|  $$$$$$$| $$ /$$$$$$$/  |  $$$$/|  $$$$$$$| $$      |  $$$$$$$|  $$$$$$$
// |__/  |__/ \_______/ \____  $$|__/|_______/    \___/   \_______/|__/       \_______/ \_______/
//                      /$$  \ $$                                                                
//                     |  $$$$$$/                                                                
//                      \______/                                                                 
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! This provides the internal `Registered` trait which must be implemented by every `Node`, and
//! can be derived through the `Register` derive macro!
//! 

use crate::services::node_registry::{ FieldMap, SFieldMap };


/// A trait which allows for the saving and loading of Nodes from owned data.
/// This trait is implemented for you via the `Registered` derive macro, which is automatically
/// set via the `class!` macro.
pub trait Registered {
    
    /// Loads a `Node` from a set of owned data in a `toml` compatible format.
    fn load_from_owned(owned_state: SFieldMap) -> Result<Self, String> where Self: Sized; /* Required for V-Table Initialization */

    /// Saves a `Node`'s owned state to a `FieldMap`, which is compatible with `Serde`.
    fn save_from_owned(&self) -> FieldMap;
}
