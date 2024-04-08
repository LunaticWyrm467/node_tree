//===================================================================================================================================================================================//
//
//  /$$$$$$$                                              /$$                 /$$$$$$$$                 /$$   /$$    
// | $$__  $$                                            |__/                |__  $$__/                |__/  | $$    
// | $$  \ $$ /$$   /$$ /$$$$$$$   /$$$$$$  /$$$$$$/$$$$  /$$  /$$$$$$$         | $$  /$$$$$$  /$$$$$$  /$$ /$$$$$$  
// | $$  | $$| $$  | $$| $$__  $$ |____  $$| $$_  $$_  $$| $$ /$$_____/         | $$ /$$__  $$|____  $$| $$|_  $$_/  
// | $$  | $$| $$  | $$| $$  \ $$  /$$$$$$$| $$ \ $$ \ $$| $$| $$               | $$| $$  \__/ /$$$$$$$| $$  | $$    
// | $$  | $$| $$  | $$| $$  | $$ /$$__  $$| $$ | $$ | $$| $$| $$               | $$| $$      /$$__  $$| $$  | $$ /$$
// | $$$$$$$/|  $$$$$$$| $$  | $$|  $$$$$$$| $$ | $$ | $$| $$|  $$$$$$$         | $$| $$     |  $$$$$$$| $$  |  $$$$/
// |_______/  \____  $$|__/  |__/ \_______/|__/ |__/ |__/|__/ \_______/         |__/|__/      \_______/|__/   \___/  
//            /$$  | $$                                                                                              
//           |  $$$$$$/                                                                                              
//            \______/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Stores the `Dynamic` trait which facilitates a type's conversion to an `Any` dynamic trait
//! object for type coercion.
//!

use std::any::Any;


pub trait Dynamic: Any {
    
    /// Returns this object as an Any trait object, allowing for type downcasting.
    fn to_any(&self) -> &dyn Any;
}

