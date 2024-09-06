//===================================================================================================================================================================================//
//
//  /$$$$$$$$           /$$ /$$$$$$$$ /$$            /$$$ /$$$$$$$$                                  /$$$$$$$           /$$             /$$                      /$$$  
// |__  $$__/          /$$/|__  $$__/|  $$          /$$_/|__  $$__/                                 | $$__  $$         |__/            | $$                     |_  $$ 
//    | $$  /$$$$$$   /$$/    | $$    \  $$        /$$/     | $$  /$$$$$$   /$$$$$$   /$$$$$$       | $$  \ $$ /$$$$$$  /$$ /$$$$$$$  /$$$$$$    /$$$$$$   /$$$$$$\  $$
//    | $$ /$$__  $$ /$$/     | $$     \  $$      | $$      | $$ /$$__  $$ /$$__  $$ /$$__  $$      | $$$$$$$//$$__  $$| $$| $$__  $$|_  $$_/   /$$__  $$ /$$__  $$| $$
//    | $$| $$  \ $$|  $$     | $$      /$$/      | $$      | $$| $$  \__/| $$$$$$$$| $$$$$$$$      | $$____/| $$  \ $$| $$| $$  \ $$  | $$    | $$$$$$$$| $$  \__/| $$
//    | $$| $$  | $$ \  $$    | $$     /$$/       |  $$     | $$| $$      | $$_____/| $$_____/      | $$     | $$  | $$| $$| $$  | $$  | $$ /$$| $$_____/| $$      /$$/
//    | $$| $$$$$$$/  \  $$   | $$    /$$/         \  $$$   | $$| $$      |  $$$$$$$|  $$$$$$$      | $$     |  $$$$$$/| $$| $$  | $$  |  $$$$/|  $$$$$$$| $$    /$$$/ 
//    |__/| $$____/    \__/   |__/   |__/           \___/   |__/|__/       \_______/ \_______/      |__/      \______/ |__/|__/  |__/   \___/   \_______/|__/   |___/  
//        | $$                                                                                                                                                         
//        | $$                                                                                                                                                         
//        |__/                                                                                                                                                         
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Provides the `Tp<T>` smart pointer which allows access to Nodes in the `NodeTree`.
//! Also provides the `TpDyn<T>` alternative to allow easy access to dynamic values.
//! 

use std::ops::{ Deref, DerefMut };
use std::any::Any;
use std::marker::PhantomData;

use super::rid::RID;
use crate::traits::{ node::Node, node_tree::NodeTree };


/*
 * Tree
 *      Pointer
 */


/// A Tree Pointer (`Tp<T>`) is a reference to a specific RID and a pointer to the `NodeTree`,
/// meaning that it has access to grab a reference or mutable reference to a Node at will.
///
/// # Lifetimes
/// This shares a LifeTime with its owning Node, as its owning Node is what manages its internal
/// pointer to the `NodeTree`.
///
/// # `Deref` and `DerefMut`
/// The Tree Pointer implements `Deref` and `DerefMut`, which automatically call the panicking
/// versions of `get()` and `get_mut()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tp<'a, T: Node> {
    node:   RID,
    tree:   *mut dyn NodeTree,
    p_life: PhantomData<&'a ()>,
    p_type: PhantomData<T>
}

impl <'a, T: Node> Tp<'a, T> {
    
    /// Creates a new `Tp<T>` via a raw pointer to the `NodeTree` and the referenced Node's `RID`.
    ///
    /// # Safety
    /// The responsibility of passing a valid pointer of the `NodeTree` to this structure is on the
    /// programmer.
    /// However, it is advised to use a `Node`'s `get_node()` or `get_node_from_tree()`
    /// function to have it be constructed in a safe manner for you!
    ///
    /// # Failure
    /// Will not return a valid `Tp<T>` pointer if the types do not match!
    pub unsafe fn new(tree: *mut dyn NodeTree, node: RID) -> Option<Self> {
        
        // First check if the types match using dynamic dispatch!
        match (&*tree).get_node(node) {
            Some(node) => {
                let any: &dyn Any = node.as_any();
                match any.downcast_ref::<T>() {
                    Some(_) => (),
                    None    => return None
                }
            },
            None => ()
        }

        Some(Tp {
            node,
            tree,
            p_life: PhantomData,
            p_type: PhantomData
        })
    }

    /// Converts this to a generic `TpDyn`.
    pub fn to_dyn(self) -> TpDyn<'a> {
        unsafe {
            TpDyn::new(self.tree, self.node)
        }
    }

    /// Determines if the `Node` this pointer is pointing to is valid.
    pub fn is_valid(&self) -> bool {
        match unsafe { &*self.tree }.get_node(self.node) {
            Some(node) => {
                let any: &dyn Any = node.as_any();
                match any.downcast_ref::<T>() {
                    Some(_) => true,
                    None    => false
                }
            },
            None => false
        }
    }
    
    /// Determines if the `Node` this pointer is pointing to is invalid.
    pub fn is_null(&self) -> bool {
        match unsafe { &*self.tree }.get_node(self.node) {
            Some(node) => {
                let any: &dyn Any = node.as_any();
                match any.downcast_ref::<T>() {
                    Some(_) => false,
                    None    => true
                }
            },
            None => true
        }
    }
    
    /// Attempts to get a reference to the underlying `Node`.
    /// # Panics
    /// Panics if the node is invalid!
    pub fn get(&self) -> &T {
        let node: Option<&dyn Node> = unsafe { &*self.tree }.get_node_raw(self.node).map(|n| unsafe { &*n });
        match node {
            Some(node) => {
                let any: &dyn Any = node.as_any();
                match any.downcast_ref::<T>() {
                    Some(node) => node,
                    None       => panic!("Invalid node!")
                }
            },
            None => panic!("Invalid Node!")
        }
    }
    
    /// Attempts to get a reference to the underlying `Node`. Returns `None` if the `Node` is invalid.
    pub fn try_get(&self) -> Option<&T> {
        let node: Option<&dyn Node> = unsafe { &*self.tree }.get_node_raw(self.node).map(|n| unsafe { &*n });
        match node {
            Some(node) => {
                let any: &dyn Any = node.as_any();
                match any.downcast_ref::<T>() {
                    Some(node) => Some(node),
                    None       => None
                }
            },
            None => None
        }
    }
    
    /// Attempts to get a mutable reference to the underlying `Node`.
    /// # Panics
    /// Panics if the node is invalid!
    pub fn get_mut(&mut self) -> &mut T {
        let node: Option<&mut dyn Node> = unsafe { &mut *self.tree }.get_node_mut_raw(self.node).map(|n| unsafe { &mut *n });
        match node {
            Some(node) => {
                let any: &mut dyn Any = node.as_any_mut();
                match any.downcast_mut::<T>() {
                    Some(node) => node,
                    None       => panic!("Invalid node!")
                }
            },
            None => panic!("Invalid Node!")
        }
    }
    
    /// Attempts to get a mutable reference to the underlying `Node`. Returns `None` if the `Node` is invalid.
    pub fn try_get_mut(&mut self) -> Option<&mut T> {
        let node: Option<&mut dyn Node> = unsafe { &mut *self.tree }.get_node_mut_raw(self.node).map(|n| unsafe { &mut *n });
        match node {
            Some(node) => {
                let any: &mut dyn Any = node.as_any_mut();
                match any.downcast_mut::<T>() {
                    Some(node) => Some(node),
                    None       => None
                }
            },
            None => None
        }
    }
}

impl <'a, T: Node> Deref for Tp<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl <'a, T: Node> DerefMut for Tp<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}


/*
 * Tree Pointer
 *      Dynamic
 */


/// A Dynamic Tree Pointer (`TpDyn`) is a reference to a specific RID and a pointer to the `NodeTree`,
/// meaning that it has access to grab a reference or mutable reference to a Node at will.
/// The difference between this and the standard `Tp<T>` is that it will allow generic access to a
/// dynamic Node object without forced coercion to a specific known node type. However, if a type
/// is later determined, it can easily be converted to a `Tp<T>` (`to<T>()`). In fact, this has a built in
/// method (`is<T>()`) to determine if this is of a specified type.
///
/// # Lifetimes
/// This shares a LifeTime with its owning Node, as its owning Node is what manages its internal
/// pointer to the `NodeTree`.
///
/// # `Deref` and `DerefMut`
/// The Tree Pointer implements `Deref` and `DerefMut`, which automatically call the panicking
/// versions of `get()` and `get_mut()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TpDyn<'a> {
    node:   RID,
    tree:   *mut dyn NodeTree,
    p_life: PhantomData<&'a ()>
}

impl <'a> TpDyn<'a> {
    
    /// Creates a new `TpDyn<T>` via a raw pointer to the `NodeTree` and the referenced Node's `RID`.
    ///
    /// # Safety
    /// The responsibility of passing a valid pointer of the `NodeTree` to this structure is on the
    /// programmer.
    /// However, it is advised to use a `Node`'s `get_node()` or `get_node_from_tree()`
    /// function to have it be constructed in a safe manner for you!
    pub unsafe fn new(tree: *mut dyn NodeTree, node: RID) -> Self {
        TpDyn {
            node,
            tree,
            p_life: PhantomData
        }
    }

    /// Converts this to a type-coerced pointer.
    pub fn to<T: Node>(self) -> Option<Tp<'a, T>> {
        unsafe {
            Tp::new(self.tree, self.node)
        }
    }

    /// Determines if this pointer references a specific type.
    pub fn is<T: Node>(&self) -> bool {
        let node: Option<&dyn Node> = unsafe { &*self.tree }.get_node_raw(self.node).map(|n| unsafe { &*n });
        match node {
            Some(node) => {
                let any: &dyn Any = node.as_any();
                match any.downcast_ref::<T>() {
                    Some(_) => true,
                    None    => false
                }
            },
            None => false
        }
    }

    /// Determines if the `Node` this pointer is pointing to is valid.
    pub fn is_valid(&self) -> bool {
        match unsafe { &*self.tree }.get_node(self.node) {
            Some(_) => true,
            None    => false
        }
    }
    
    /// Determines if the `Node` this pointer is pointing to is invalid.
    pub fn is_null(&self) -> bool {
        match unsafe { &*self.tree }.get_node(self.node) {
            Some(_) => false,
            None    => true
        }
    }
    
    /// Attempts to get a reference to the underlying `Node`.
    /// # Panics
    /// Panics if the node is invalid!
    pub fn get(&self) -> &dyn Node {
        let node: Option<&dyn Node> = unsafe { &*self.tree }.get_node_raw(self.node).map(|n| unsafe { &*n });
        match node {
            Some(node) => node,
            None       => panic!("Invalid Node!")
        }
    }
    
    /// Attempts to get a reference to the underlying `Node`. Returns `None` if the `Node` is invalid.
    pub fn try_get(&self) -> Option<&dyn Node> {
        unsafe { &*self.tree }.get_node_raw(self.node).map(|n| unsafe { &*n })
    }
    
    /// Attempts to get a mutable reference to the underlying `Node`.
    /// # Panics
    /// Panics if the node is invalid!
    pub fn get_mut(&mut self) -> &mut dyn Node {
        let node: Option<&mut dyn Node> = unsafe { &mut *self.tree }.get_node_mut_raw(self.node).map(|n| unsafe { &mut *n });
        match node {
            Some(node) => node,
            None       => panic!("Invalid Node!")
        }
    }
    
    /// Attempts to get a mutable reference to the underlying `Node`. Returns `None` if the `Node` is invalid.
    pub fn try_get_mut(&mut self) -> Option<&mut dyn Node> {
        unsafe { &mut *self.tree }.get_node_mut_raw(self.node).map(|n| unsafe { &mut *n })
    }
}

impl <'a> Deref for TpDyn<'a> {
    type Target = dyn Node;
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl <'a> DerefMut for TpDyn<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}
