use std::sync::MutexGuard;

use crate::MutableArc;
use crate::structs::{ node_tree::{ NodeTree, ProcessMode }, node_query::NodeQuery };
use super::dynamic::Dynamic;


mod private {
    use crate::structs::{ node_path::NodePath, node_query::NodeQuery };
    use crate::utils::functions::ensure_unique_name;
    use super::{ NodeAbstract, DynNode, NodeMutex };

    
    /// Contains sealed methods that should not be overriden for the Node trait.
    pub trait NodeSealed: NodeAbstract {
        
        /// Sets the name of the node.
        /// This will fail if the name is not unique within the context of the parent's children
        /// vector.
        /// Returns false if the operation fails.
        fn set_name(&mut self, name: &str) -> bool {
            if let NodeQuery::Some(parent) = self.parent() {
                let mut is_unique: bool         = true;
                let     neighbors: Vec<DynNode> = parent.lock().unwrap().children().iter().map(|a| a.to_owned()).collect();

                for neighbor in neighbors {
                    let neighbor_name: String = neighbor.lock().unwrap().name().to_string();
                    if &neighbor_name == self.name() {
                        continue;
                    }
                    if &neighbor_name == name {
                        is_unique = false;
                        break;
                    }
                }

                if is_unique {
                    unsafe {
                        self.set_name_unchecked(name);
                    }
                }
                is_unique
            } else {
                unsafe {
                    self.set_name_unchecked(name);
                }
                true
            }
        }

        /// Returns true if this node is a stray node with no parent or owner.
        fn is_stray(&self) -> bool {
            self.parent().is_none() && self.owner().is_none()
        }

        /// Returns true if this node is the root node.
        fn is_root(&self) -> bool {
            self.parent().is_none() && self.in_tree()
        }

        /// Returns if this node is a part of the node tree.
        fn in_tree(&self) -> bool {
            self.owner().is_some()
        }

        /// Returns the number of children this node has.
        fn num_children(&self) -> usize {
            self.children().len()
        }

        /// Returns true if this node has no children.
        fn has_no_children(&self) -> bool {
            self.num_children() == 0
        }

        /// Adds a child to the node, automatically renaming it if its name is not unique in the
        /// node's children vector.
        /// If this node is connected to the node tree, then `_ready()` will automatically be
        /// propogated throughout its ranks.
        fn add_child(&mut self, node: DynNode) -> () {

            // Ensure that the child's name within the context of this node's children is unique.
            let     names_of_children: Vec<String> = self.children().iter().map(|c| c.lock().unwrap().name().to_string()).collect();
            let mut node_locked:       NodeMutex   = node.lock().unwrap();
            let     node_name:         String      = node_locked.name().to_string();

            unsafe {
                node_locked.set_name_unchecked(&ensure_unique_name(&node_name, names_of_children))
            }
            drop(node_locked);

            // Add the child to this node's children and connect it to its parent and owner nodes.
            unsafe {
                node.lock().unwrap().set_parent(self.as_dyn());
            }
            self.children_mut().push(node);

            if self.in_tree() {
                let mut child: NodeMutex = self.children()[self.num_children() - 1].lock().unwrap();
                unsafe {
                    child.set_owner(self.owner().unwrap());   // For now, we just propagate the root as the owner for all nodes.
                }
                for node in child.bottom_up() {
                    node.lock().unwrap().ready();
                }
            }
        }

        /// Returns a child at the given index.
        /// If there is no child at the given index, then the NodeQuery will be empty.
        fn get_child(&self, i: usize) -> NodeQuery {
            if i >= self.num_children() {
                NodeQuery::None
            } else {
                NodeQuery::Some(self.children()[i].to_owned())
            }
        }

        /// Gets a node given a NodePath that is respective to this node as the root.
        fn get_node(&self, mut path: NodePath) -> NodeQuery {
            let next_node: Option<String> = path.pop_front();
            match next_node {
                Some(target) => {
                    for node in self.children() {
                        let node_unlocked: NodeMutex = node.lock().unwrap();
                        if node_unlocked.name() == target {
                            return node_unlocked.get_node(path);
                        }
                    }
                    NodeQuery::None
                },
                None => NodeQuery::Some(self.as_dyn())
            }
        }

        /// Produces a normal top-down order iteration of all of the nodes connected to this node.
        /// This is used to handle a lot of the scene tree behaviour.
        /// # Note
        /// Nodes that are at the beginning of the children vector will be prioritized.
        fn top_down(&mut self) -> Vec<DynNode> {
            let mut iter: Vec<DynNode> = Vec::new();
            self.top_down_tail(&mut iter);
            iter
        }

        /// The tail end recursive function for the `top_down` method.
        fn top_down_tail(&mut self, iter: &mut Vec<DynNode>) -> () {
            *iter = iter.iter().chain(self.children()).map(|a| a.to_owned()).collect();
            for child in self.children() {
                *iter = iter.iter().chain(child.lock().unwrap().children()).map(|a| a.to_owned()).collect();
            }
        }

        /// Produces a reverse bottom-up order iteration of all of the nodes connected to this node.
        /// This is typically used to initialize nodes or scenes of nodes.
        fn bottom_up(&mut self) -> Vec<DynNode> {
            let mut iter:  Vec<DynNode> = Vec::new();
            let     layer: Vec<DynNode> = self.gather_deepest();
            self.bottom_up_tail(&mut iter, layer);
            iter
        }

        /// This gathers the deepest nodes in the tree.
        fn gather_deepest(&mut self) -> Vec<DynNode> {
            let mut deepest_nodes: Vec<DynNode> = Vec::new();
            for node in self.children() {
                deepest_nodes.append(&mut node.lock().unwrap().gather_deepest());
            }
            deepest_nodes
        }

        /// The tail end recursive function for the `bottom_up` method.
        /// Due to how this functions, this function call doesn't actually call itself on different
        /// layers of the node tree, but it rather calls itself.
        fn bottom_up_tail(&mut self, iter: &mut Vec<DynNode>, layer: Vec<DynNode>) -> () {

            // Define a function to filter out duplicates.
            fn filter_duplicates(arr: Vec<DynNode>) -> Vec<DynNode> {
                let mut unique: Vec<DynNode> = Vec::new();
                for item in arr {
                    let mut is_unique: bool = true;

                    for unique_item in &unique {
                        let item_name:   String = item.lock().unwrap().name().to_string();
                        let unique_name: String = unique_item.lock().unwrap().name().to_string();

                        if item_name == unique_name {
                            is_unique = false;
                            break;
                        }
                    }

                    if is_unique {
                        unique.push(item);
                    }
                }
                unique
            }

            // We get the next layer by getting the node's parents and filtering out duplicates.
            let next_layer: Vec<DynNode> = filter_duplicates(layer.iter().map(|node| node.lock().unwrap().parent().unwrap()).collect());
            for node in layer {
                iter.push(node);
            }

            // If the next layer is only made up of one node and said node has the same name as this
            // node, then return.
            if next_layer.len() == 1 && next_layer[0].lock().unwrap().name() == self.name() {
                return;
            }

            self.bottom_up_tail(iter, next_layer);
        }
    }
}
impl <T: NodeAbstract> private::NodeSealed for T {}


/// In order to make the code more readable, we use this type name instead of MutableArc<dyn Node>.
pub type DynNode = MutableArc<dyn Node>;

/// In order to make the code more readable, we use this type name instead of MutexGuard<dyn Node>.
pub type NodeMutex<'a> = MutexGuard<'a, dyn Node>;


/// This implements of of the node's abstract behaviours.
/// This, along with `Node` must be implemented in order to create a new node.
pub trait NodeAbstract: Dynamic + Send + Sync {
    
    /// Gets this as a dynamic Node object.
    fn as_dyn(&self) -> DynNode;

    /// Gets the name of the node.
    /// Each name must be unique within the context of the parent's children vector.
    fn name(&self) -> &str;

    /// Sets the name of the node without checking if the name is unique.
    /// This should only be implemented, but not used manually.
    unsafe fn set_name_unchecked(&mut self, name: &str) -> ();

    /// Gets the reference to the root NodeTree structure, which controls the entire tree.
    fn root(&self) -> MutableArc<NodeTree>;

    /// Gets the owner of the node.
    /// The owner is different from the parent. The owner can be thought as the root of the scene
    /// that this node is a part of, rather than the node's actual parent.
    /// In other words, if you had a node tree that looked like this:
    /// ```text
    /// ... <Higher Nodes>
    /// ╰NodeA <Root of Saved Scene>
    ///  ├NodeB
    ///  ╰NodeC
    ///   ╰NodeD
    ///```
    /// And you were to call `owner()` on `NodeD`, you would get `NodeA`.
    /// # Note
    /// You can only have an owner on a node that is a part of a node tree.
    fn owner(&self) -> NodeQuery;

    /// Sets the owner of the node.
    /// This should only be implemented, but not used manually.
    unsafe fn set_owner(&mut self, owner: DynNode) -> ();

    /// Gets the direct parent of this node.
    fn parent(&self) -> NodeQuery;

    /// Sets the parent of this node.
    /// This should only be implemented, but not used manually.
    unsafe fn set_parent(&mut self, parent: DynNode) -> ();


    /// Gets a vector of this node's children.
    fn children(&self) -> &Vec<DynNode>;
    
    /// Gets a mutable vector of this node's children.
    fn children_mut(&self) -> &mut Vec<DynNode>;
}


/// This only holds the node's 'programmable' behaviours.
/// This must be implemented along with `NodeAbstract` to create a new node.
pub trait Node: NodeAbstract + private::NodeSealed {
    
    /// This function can be overridden to facilitate this node's starting behaviour.
    /// This only runs once after the scene that the node is a part of is fully initialized.
    fn ready(&mut self) -> ();

    /// This function can be overridden to facilitate behaviour that must update on a timely
    /// manner.
    /// This runs once per tick, and returns a delta value capturing the time between frames.
    fn process(&mut self, _delta: f32) -> ();

    /// This function can be overrriden to facilitate this node's terminal behaviour.
    /// It is run immeditately after this node is queued for destruction.
    fn terminal(&mut self) -> ();

    /// This returns the node's process mode, and entirely effects how the process() function
    /// behaves.
    /// By default, this returns `Inherit`.
    /// # Note
    /// Any node at the root of the scene tree with the `Inherit` property will by default inherit
    /// the `Pausable` process mode.
    fn process_mode(&self) -> ProcessMode {
        ProcessMode::Inherit
    }
}
