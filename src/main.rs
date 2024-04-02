pub mod structs;
pub mod traits;
pub mod utils;

use std::sync::{ Arc, Mutex };

pub type MutableArc<T> = Arc<Mutex<T>>;

// TODO:
// - Hook up the owner nodes during scene initialization before ready() is called.
// - Have the parent be connected when a child node is added.
// - Implement global ready() and process() calls in the Node Tree.


fn main() {
    use utils::functions::ensure_unique_name;

    let test_scene: Vec<String> = vec!["Hello8", "Hello9", "Hello10", "Hello11", "Hello12"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let new_name:   &str        = "Hello9";

    println!("{}", ensure_unique_name(new_name, test_scene));
}
