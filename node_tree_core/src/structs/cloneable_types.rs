//===================================================================================================================================================================================//
//
//   /$$$$$$  /$$                                         /$$       /$$                 /$$$$$$$$                                     
//  /$$__  $$| $$                                        | $$      | $$                |__  $$__/                                     
// | $$  \__/| $$  /$$$$$$  /$$$$$$$   /$$$$$$   /$$$$$$ | $$$$$$$ | $$  /$$$$$$          | $$ /$$   /$$  /$$$$$$   /$$$$$$   /$$$$$$$
// | $$      | $$ /$$__  $$| $$__  $$ /$$__  $$ |____  $$| $$__  $$| $$ /$$__  $$         | $$| $$  | $$ /$$__  $$ /$$__  $$ /$$_____/
// | $$      | $$| $$  \ $$| $$  \ $$| $$$$$$$$  /$$$$$$$| $$  \ $$| $$| $$$$$$$$         | $$| $$  | $$| $$  \ $$| $$$$$$$$|  $$$$$$ 
// | $$    $$| $$| $$  | $$| $$  | $$| $$_____/ /$$__  $$| $$  | $$| $$| $$_____/         | $$| $$  | $$| $$  | $$| $$_____/ \____  $$
// |  $$$$$$/| $$|  $$$$$$/| $$  | $$|  $$$$$$$|  $$$$$$$| $$$$$$$/| $$|  $$$$$$$         | $$|  $$$$$$$| $$$$$$$/|  $$$$$$$ /$$$$$$$/
//  \______/ |__/ \______/ |__/  |__/ \_______/ \_______/|_______/ |__/ \_______/         |__/ \____  $$| $$____/  \_______/|_______/ 
//                                                                                             /$$  | $$| $$                          
//                                                                                            |  $$$$$$/| $$                          
//                                                                                             \______/ |__/                          
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Provides the Default on Clone (`Doc<T>`), Error on Clone (`Eoc<T>`), and Void on Clone (`Voc<T>`)
//! types to help with clone assertions.
//! 

use std::ops::{ Deref, DerefMut };


/// `Doc<T>` (Default on Clone), simply returns the default type when cloned.
#[derive(Debug, Default)]
pub struct Doc<T: Default>(T);

impl <T: Default> Doc<T> {
    
    /// Creates a new `Doc<T>`.
    pub fn new(item: T) -> Self {
        Doc(item)
    }
}

impl <T: Default> Clone for Doc<T> {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl <T: Default> Deref for Doc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <T: Default> DerefMut for Doc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


/// `Eoc<T>` (Error on Clone), simply causes the panic when cloned.
#[derive(Debug)]
pub struct Eoc<T>(T);

impl <T> Eoc<T> {
    
    /// Creates a new `Eoc<T>`.
    pub fn new(item: T) -> Self {
        Eoc(item)
    }
}

impl <T> Clone for Eoc<T> {
    fn clone(&self) -> Self {
        panic!("Attempted to clone an Eoc<T> (Error on Clone)!");
    }
}

impl <T> Deref for Eoc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <T> DerefMut for Eoc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


/// `Voc<T>` (Void on Clone), simply returns a `Null` state when cloned, which can be inferred upon
/// in a non-panicking manner.
#[derive(Debug)]
pub enum Voc<T> {
    Valid(T),
    Void
}

impl <T> Voc<T> {
    
    /// Creates a new `Voc<T>`.
    pub fn new(item: T) -> Self {
        Voc::Valid(item)
    }

    /// Checks if the container's item is valid.
    pub fn is_reacheable(&self) -> bool {
        match self {
            Self::Valid(_) => true,
            Self::Void     => false
        }
    }

    /// Checks if the container's item has been voided.
    pub fn is_void(&self) -> bool {
        match self {
            Self::Valid(_) => false,
            Self::Void     => true
        }
    }
}

impl <T> Clone for Voc<T> {
    fn clone(&self) -> Self {
        Self::Void
    }
}

impl <T> Deref for Voc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Valid(ref valid) => valid,
            Self::Void             => panic!("Attempted to utilize a voided Voc<T> (Void on Clone)!")
        }
    }
}

impl <T> DerefMut for Voc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Valid(ref mut valid) => valid,
            Self::Void                 => panic!("Attempted to utilize a voided Voc<T> (Void on Clone)!")
        }
    }
}
