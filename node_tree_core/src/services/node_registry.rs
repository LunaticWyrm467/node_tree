//===================================================================================================================================================================================//
//
//  /$$   /$$                 /$$                 /$$$$$$$                      /$$             /$$                        
// | $$$ | $$                | $$                | $$__  $$                    |__/            | $$                        
// | $$$$| $$  /$$$$$$   /$$$$$$$  /$$$$$$       | $$  \ $$  /$$$$$$   /$$$$$$  /$$  /$$$$$$$ /$$$$$$    /$$$$$$  /$$   /$$
// | $$ $$ $$ /$$__  $$ /$$__  $$ /$$__  $$      | $$$$$$$/ /$$__  $$ /$$__  $$| $$ /$$_____/|_  $$_/   /$$__  $$| $$  | $$
// | $$  $$$$| $$  \ $$| $$  | $$| $$$$$$$$      | $$__  $$| $$$$$$$$| $$  \ $$| $$|  $$$$$$   | $$    | $$  \__/| $$  | $$
// | $$\  $$$| $$  | $$| $$  | $$| $$_____/      | $$  \ $$| $$_____/| $$  | $$| $$ \____  $$  | $$ /$$| $$      | $$  | $$
// | $$ \  $$|  $$$$$$/|  $$$$$$$|  $$$$$$$      | $$  | $$|  $$$$$$$|  $$$$$$$| $$ /$$$$$$$/  |  $$$$/| $$      |  $$$$$$$
// |__/  \__/ \______/  \_______/ \_______/      |__/  |__/ \_______/ \____  $$|__/|_______/    \___/  |__/       \____  $$
//                                                                    /$$  \ $$                                   /$$  | $$
//                                                                   |  $$$$$$/                                  |  $$$$$$/
//                                                                    \______/                                    \______/ 
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Provides and initializes a global state which keeps track of `Node` deserializer functions.
//! 

use std::sync::Arc;
use std::collections::HashMap;

use dashmap::DashMap;

pub use toml::Value;

use crate::traits::node::Node;
use crate::traits::serializable::Serializable;


/// Represents a static node registry.
/// This contains an ID which is represented as the Node's name, followed by a function pointer
/// which is used to convert a map which is parsed from a scene file back into its respective node.
///
/// # Note
/// The reason why we use the name and not the `TypeID` is because the `TypeID` can change across rust
/// versions and builds, while the name of the node is fully controlled by the programmer.
/// This however also has its downsides, such as the moving of the module that a node is defined
/// in, or renaming the node itself resulting in the invalidation of prior save data.
///
/// Furthermore, the interesting thing about using a `HashMap<Box<str>, Box<dyn Serializable>>`/`HashMap<Box<str>, toml::Value>
/// as the opaque representation for a `Node`'s owned fields is that a derive macro can easily generate
/// the deserialization function and register it to the `NODE_REGISTRY`. An example of a generate
/// function is shown below:
/// ```rust, ignore
/// class! {
///     dec NodeName;
///
///     let field_a: String;
///     let field_b: u32;
///     let field_c: Vec<f32>;
/// }
///
/// // This function was generated for `NodeName` via the `Register` derive macro, which is
/// implemented via `class!`:
/// fn deserialize(owned_state: SFieldMap) -> Result<Box<dyn Node>, String> {
///     let node: NodeName = NodeName::load_from_owned(
///         String::from_value(*owned_state.remove("field_a").ok_or("corrupt save data; `field_a` missing".to_string())?).ok_or("corrupt save data; `field_a` invalid type".to_string())?,
///         u32::from_value(*owned_state.remove("field_b").ok_or("corrupt save data; `field_b` missing".to_string())?).ok_or("corrupt save data; `field_b` invalid type".to_string())?,
///         Vec::from_value(*owned_state.remove("field_c").ok_or("corrupt save data; `field_c` missing".to_string())?).ok_or("corrupt save data; `field_c` invalid type".to_string())?,
///     );
///     Box::new(node)
/// }
/// ```
//#[dynamic]
static mut NODE_REGISTRY: Option<Arc<Registry>> = None;

/// Used as a alias for a map containing the unserialized fields of a node, along with its associated values.
pub type FieldMap = HashMap<Box<str>, Box<dyn Serializable>>;

/// Used as an alias for a map containing the serialized fields of a node, along with its associated values.
pub type SFieldMap = HashMap<Box<str>, Value>;

/// Used as an alias for the deserializer function.
pub type Deserializer = dyn Fn(SFieldMap) -> Result<Box<dyn Node>, String>;

/// Used as an unsafe cell for the registry to allow for the sharing of values.
struct Registry {
    registry: DashMap<Box<str>, Box<Deserializer>>
}

unsafe impl Send for Registry {}
unsafe impl Sync for Registry {}

/// Registers a deserializing function under a node's name.
///
/// # Safety
/// This should only be called from the main thread or from one thread at a time before the main
/// function is invoked via `ctor`.
pub unsafe fn register_deserializer(name: Box<str>, deserializer: impl Fn(SFieldMap) -> Result<Box<dyn Node>, String> + 'static) {
    if NODE_REGISTRY.is_none() {
        NODE_REGISTRY = Some(Arc::new(Registry { registry: DashMap::new() }));
    }
    NODE_REGISTRY.as_mut().unwrap().registry.insert(name, Box::new(deserializer));
}

/// Takes a `SFieldMap` and deserializes it into a `Node` with a bare `NodeBase`.
pub fn deserialize(name: &str, owned_state: SFieldMap) -> Result<Box<dyn Node>, String> {
    
    // Safety:
    // This does not mutate state and `register_deserializer`, which does mutate state, is marked
    // unsafe and is expected to run before the main function is invoked.
    unsafe {
        (NODE_REGISTRY.as_ref()
         .ok_or("attempting to deserialize from an unregistered node".to_string())?
         .registry
         .get(name)
         .ok_or("attempting to deserialize from an unregistered node".to_string()))?
            (owned_state)
    }
}
