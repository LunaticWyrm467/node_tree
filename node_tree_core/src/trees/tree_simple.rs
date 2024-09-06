//===================================================================================================================================================================================//
//
//   /$$$$$$  /$$                         /$$                 /$$$$$$$$                           
//  /$$__  $$|__/                        | $$                |__  $$__/                           
// | $$  \__/ /$$ /$$$$$$/$$$$   /$$$$$$ | $$  /$$$$$$          | $$  /$$$$$$   /$$$$$$   /$$$$$$ 
// |  $$$$$$ | $$| $$_  $$_  $$ /$$__  $$| $$ /$$__  $$         | $$ /$$__  $$ /$$__  $$ /$$__  $$
//  \____  $$| $$| $$ \ $$ \ $$| $$  \ $$| $$| $$$$$$$$         | $$| $$  \__/| $$$$$$$$| $$$$$$$$
//  /$$  \ $$| $$| $$ | $$ | $$| $$  | $$| $$| $$_____/         | $$| $$      | $$_____/| $$_____/
// |  $$$$$$/| $$| $$ | $$ | $$| $$$$$$$/| $$|  $$$$$$$         | $$| $$      |  $$$$$$$|  $$$$$$$
//  \______/ |__/|__/ |__/ |__/| $$____/ |__/ \_______/         |__/|__/       \_______/ \_______/
//                             | $$                                                               
//                             | $$                                                               
//                             |__/                                                               

//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! Contains a simply implementation of a `NodeTree`.
//!

use std::any::Any;
use std::ops::{ Deref, DerefMut };

use crate::structs::logger::LoggerVerbosity;
use crate::structs::node_tree_base::NodeTreeBase;
use crate::traits::{ node::Node, node_tree::{ NodeTree, init_base } };


/// A simple implementation of a `NodeTree` which will work just fine for most applications that do
/// not make use of other frameworks.
#[derive(Debug)]
pub struct TreeSimple {
    base: Option<NodeTreeBase>
}

impl TreeSimple {
    
    /// Creates a new `TreeSimple` structure.
    pub fn new<N: Node>(root: N, verbosity: LoggerVerbosity) -> Box<Self> {
        let mut tree: Box<TreeSimple> = Box::new(TreeSimple {
            base: None
        });
        
        init_base(&mut tree, root, verbosity);
        tree
    }
}

impl NodeTree for TreeSimple {
    unsafe fn set_base(&mut self, base: NodeTreeBase) {
        self.base = Some(base);
    }

    fn base(&self) -> &NodeTreeBase {
        unsafe {
            self.base.as_ref().unwrap_unchecked()
        }
    }

    fn base_mut(&mut self) -> &mut NodeTreeBase {
        unsafe {
            self.base.as_mut().unwrap_unchecked()
        }
    }

    fn as_dyn(&self) -> &dyn NodeTree {
        self
    }

    fn as_dyn_mut(&mut self) -> &mut dyn NodeTree {
        self
    }

    fn as_dyn_raw(&self) -> *const dyn NodeTree {
        self
    }

    fn as_dyn_raw_mut(&mut self) -> *mut dyn NodeTree {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Deref for TreeSimple {
    type Target = NodeTreeBase;
    fn deref(&self) -> &Self::Target {
        unsafe {
            self.base.as_ref().unwrap_unchecked()
        }
    }
}

impl DerefMut for TreeSimple {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.base.as_mut().unwrap_unchecked()
        }
    }
}
