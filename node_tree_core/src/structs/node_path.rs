//===================================================================================================================================================================================//
//
//  /$$   /$$                 /$$                 /$$$$$$$             /$$     /$$      
// | $$$ | $$                | $$                | $$__  $$           | $$    | $$      
// | $$$$| $$  /$$$$$$   /$$$$$$$  /$$$$$$       | $$  \ $$ /$$$$$$  /$$$$$$  | $$$$$$$ 
// | $$ $$ $$ /$$__  $$ /$$__  $$ /$$__  $$      | $$$$$$$/|____  $$|_  $$_/  | $$__  $$
// | $$  $$$$| $$  \ $$| $$  | $$| $$$$$$$$      | $$____/  /$$$$$$$  | $$    | $$  \ $$
// | $$\  $$$| $$  | $$| $$  | $$| $$_____/      | $$      /$$__  $$  | $$ /$$| $$  | $$
// | $$ \  $$|  $$$$$$/|  $$$$$$$|  $$$$$$$      | $$     |  $$$$$$$  |  $$$$/| $$  | $$
// |__/  \__/ \______/  \_______/ \_______/      |__/      \_______/   \___/  |__/  |__/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! NodePaths are processed string paths to a node from a given starting node.
//!
//! # Example
//! Lets assume that you have the following scene set up:
//! ```text
//! ... <Higher Nodes>
//! ╰NodeA <Calling Node>
//!  ├NodeB
//!  ╰NodeC
//!   ╰NodeD
//!```
//! You want to get NodeD from NodeA, so you would therefore do something like this:
//! ```rust,ignore
//! use node_tree::prelude::*;
//!
//! fn example(node_a: &dyn Node) {
//!     let path_d: NodePath         = nodepath!("NodeC/NodeD"); // Syntax sugar for NodePath::from_str()
//!     let node_d: Tp<YourNodeType> = node_a.get_node(path_d).unwrap();
//!     // ... Do whatever
//! }
//! ```
//!
//! More examples of valid path types can be found under the documentation of `NodePath`.

use std::fmt;
use std::collections::VecDeque;

use crate::traits::node_getter::NodeGetter;
use super::{ node_tree_base::NodeTreeBase, rid::RID };


/*
 * Path
 *      Segment
 */


/// A path segment used to denote either node names, or special identifiers.
#[derive(Clone, Hash, PartialEq, Eq)]
pub(crate) enum PathSeg {
    Node(Box<str>), // Any other identifier`
    This,           // `.`
    Parent          // `..
}

impl PathSeg {
    
    /// Creates a PathSeg from a string literal.
    fn parse(input: &str) -> Self {
        match input {
            "."   => Self::This,
            ".."  => Self::Parent,
            i @ _ => Self::Node(i.into())
        }
    }

    /// Returns if this is an empty `Node` identifier.
    #[inline]
    fn is_empty_identifier(&self) -> bool {
        if let Self::Node(str) = self {
            return str.is_empty();
        }
        false
    }

    /// Converts this back to a string.
    pub fn to_string(&self) -> String {
        match self {
            Self::Node(str) => str.to_string(),
            Self::This      => ".".to_string(),
            Self::Parent    => "..".to_string(),
        }
    }
}


/*
 * Node
 *      Path
 */


/// A NodePath is a parsed string that holds a map for the NodeTree to follow and to retrieve
/// a given node.
///
/// The following path types are valid:
/// ```rust,ignore
/// let to_child:       NodePath = nodepath!("A");
/// let to_grandchild:  NodePath = nodepath!("A/B");
/// let to_self:        NodePath = nodepath!(".");
/// let to_parent:      NodePath = nodepath!("..");
/// let to_sibling:     NodePath = nodepath!("../C");
/// let to_grandparent: NodePath = nodepath!("../..");
/// ```
/// Furthermore, absolute node paths can be declared with a simple leading slash, like so:
/// ```rust, ignore
/// let root: NodePath = nodepath!("/root");
/// ```
#[derive(Clone, Default, Hash, PartialEq, Eq)]
pub struct NodePath {
    path: VecDeque<PathSeg>,
    abs:  bool
}

impl NodePath {

    /// Creates an empty NodePath.
    pub fn new() -> NodePath {
        NodePath {
            path: VecDeque::new(),
            abs:  false
        }
    }
    
    /// Creates an empty absolute NodePath.
    pub fn new_abs() -> NodePath {
        NodePath {
            path: VecDeque::new(),
            abs:  true
        }
    }
    
    /// Creates a new NodePath from a string.
    /// In the string, each node name must be seperated by a `/`, like so:
    /// ```text
    /// "node_a/node_b/node_c/target_node"
    /// ```
    pub fn from_str(str: &str) -> NodePath {
        let mut path: VecDeque<PathSeg> = str.split('/').map(PathSeg::parse).collect();
        let     abs:  bool              = path.front().map(|f| f.is_empty_identifier()).unwrap_or(false);
        
        if abs {
            path.pop_front();
        }

        let mut np: NodePath = NodePath {
            path,
            abs
        };
        np.scan();
        np
    }

    /// Converts the path to a string, consuming it.
    pub fn to_string(mut self) -> String {
        let mut out: String = String::new();
        while let Some(segment) = self.pop_front() {
            out += &(segment.to_string() + "/");
        }
        out.get(0..(out.len() - 1)).unwrap().to_string()
    }

    /// Adds a node to the back of the path.
    #[inline]
    pub fn add_node(&mut self, node_name: &str) {
        self.path.push_back(PathSeg::parse(node_name));
        self.scan();
    }

    /// Pops the front-most node off the path and returns it, if there is one.
    /// If the path is empty, then this returns None.
    #[inline]
    pub(crate) fn pop_front(&mut self) -> Option<PathSeg> {
        self.path.pop_front()
    }

    /// Pops the front-most node off the path as a string and returns it, if there is one.
    /// If the path is empty, then this returns None.
    #[inline]
    pub fn pop_front_as_string(&mut self) -> Option<String> {
        self.path.pop_front().map(|seg| seg.to_string())
    }

    /// Returns whether this `NodePath` is absolute or not.
    pub fn is_absolute(&self) -> bool {
        self.abs
    }

    /// All this function does is condenses empty identifiers (`//`) into a single slash (`/`).
    #[inline]
    fn scan(&mut self) {
        self.path.retain(|seg| !seg.is_empty_identifier());
    }
}

impl NodeGetter for NodePath {
    fn get_from(&self, tree: &NodeTreeBase, caller: Option<RID>) -> Option<RID> {
        if !self.is_absolute() {
            return tree.get_node(caller?)?.get_node_raw(self.clone());
        }
        
        let mut absolute_path: NodePath = self.clone();
        if Some(tree.root().name()) != absolute_path.pop_front_as_string().as_deref() {
            return None;
        }
        tree.root().get_node_raw(absolute_path)
    }
}

impl fmt::Debug for NodePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut path: String = self.path.iter().map(|node| "/".to_owned() + &node.to_string()).collect();
                path         = "'".to_string() + &path + "'";

        f.write_str(&path)?;
        Ok(())
    }
}

/// A simple macro which is compatible with Rust's format syntax used in macros like `print!`,
/// `println!`, and `format!`.
/// Creates a `NodePath from the passed in syntax.
#[macro_export]
macro_rules! nodepath {
    ($fmt_str:literal) => {{
        NodePath::from_str(&format!($fmt_str))
    }};

    ($fmt_str:literal, $($args:expr),*) => {{
        NodePath::from_str(&format!($fmt_str, $($args),*))
    }};
}
