use std::sync::MutexGuard;

use crate::MutableArc;
use crate::structs::{ node_tree::{ NodeTree, ProcessMode }, node_path::NodePath, node_query::NodeQuery };
use crate::utils::functions::ensure_unique_name;
use super::dynamic::Dynamic;


/// This implements of of the node's abstract behaviours.
/// This, along with `Node` must be implemented in order to create a new node.
pub trait NodeAbstract: Dynamic + Send + Sync {
    
    /// Gets this as a dynamic Node object.
    fn as_dyn(&self) -> MutableArc<dyn Node>;

    /// Gets the name of the node.
    /// Each name must be unique within the context of the parent's children vector.
    fn name(&self) -> &str;

    /// Sets the name of the node.
    /// This will fail if the name is not unique within the context of the parent's children
    /// vector.
    /// Returns false if the operation fails.
    fn set_name(&mut self, name: &str) -> bool {
        if let NodeQuery::Some(parent) = self.parent() {
            let mut is_unique: bool                      = true;
            let     neighbors: Vec<MutableArc<dyn Node>> = parent.lock().unwrap().children().iter().map(|a| a.to_owned()).collect();
            
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

    /// Sets the name of the node without checking if the name is unique.
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

    /// Gets the direct parent of this node.
    fn parent(&self) -> NodeQuery;

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

    /// Gets a vector of this node's children.
    fn children(&self) -> &Vec<MutableArc<dyn Node>>;
    
    /// Gets a mutable vector of this node's children.
    fn children_mut(&self) -> &mut Vec<MutableArc<dyn Node>>;

    /// Returns the number of children this node has.
    fn num_children(&self) -> usize {
        self.children().len()
    }

    /// Returns true if this node has no children.
    fn has_no_children(&self) -> bool {
        self.num_children() == 0
    }

    /// Adds a child to the node.
    /// If this node is connected to the node tree, then `_ready()` will automatically be
    /// propogated throughout its ranks.
    fn add_child(&mut self, node: MutableArc<dyn Node>) -> () {
        let     names_of_children: Vec<String>          = self.children().iter().map(|c| c.lock().unwrap().name().to_string()).collect();
        let mut node_locked:       MutexGuard<dyn Node> = node.lock().unwrap();
        let     node_name:         String               = node_locked.name().to_string();
        
        unsafe {
            node_locked.set_name_unchecked(&ensure_unique_name(&node_name, names_of_children))
        }
        drop(node_locked);

        self.children_mut().push(node);
        if self.in_tree() {
            for node in self.children()[self.num_children() - 1].lock().unwrap().bottom_up() {
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
                    let node_unlocked: MutexGuard<dyn Node> = node.lock().unwrap();
                    if  node_unlocked.name() == target {
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
    fn top_down(&mut self) -> Vec<MutableArc<dyn Node>> {
        let mut iter: Vec<_> = Vec::new();
        self.top_down_tail(&mut iter);
        iter
    }

    /// The tail end recursive function for the `top_down` method.
    fn top_down_tail(&mut self, iter: &mut Vec<MutableArc<dyn Node>>) -> () {
        *iter = iter.iter().chain(self.children()).map(|a| a.to_owned()).collect();
        for child in self.children() {
            *iter = iter.iter().chain(child.lock().unwrap().children()).map(|a| a.to_owned()).collect();
        }
    }

    /// Produces a reverse bottom-up order iteration of all of the nodes connected to this node.
    /// This is typically used to initialize nodes or scenes of nodes.
    fn bottom_up(&mut self) -> Vec<MutableArc<dyn Node>> {
        let mut iter:  Vec<_> = Vec::new();
        let     layer: Vec<_> = self.gather_deepest();
        self.bottom_up_tail(&mut iter, layer);
        iter
    }

    /// This gathers the deepest nodes in the tree.
    fn gather_deepest(&mut self) -> Vec<MutableArc<dyn Node>> {
        let mut deepest_nodes: Vec<MutableArc<dyn Node>> = Vec::new();
        for node in self.children() {
            deepest_nodes.append(&mut node.lock().unwrap().gather_deepest());
        }
        deepest_nodes
    }

    /// The tail end recursive function for the `bottom_up` method.
    /// Due to how this functions, this function call doesn't actually call itself on different
    /// layers of the node tree, but it rather calls itself.
    fn bottom_up_tail(&mut self, iter: &mut Vec<MutableArc<dyn Node>>, layer: Vec<MutableArc<dyn Node>>) -> () {

        // Define a function to filter out duplicates.
        fn filter_duplicates(arr: Vec<MutableArc<dyn Node>>) -> Vec<MutableArc<dyn Node>> {
            let mut unique: Vec<MutableArc<dyn Node>> = Vec::new();
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
        let next_layer: Vec<MutableArc<dyn Node>> = filter_duplicates(layer.iter().map(|node| node.lock().unwrap().parent().unwrap()).collect());
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

/// This only holds the node's 'programmable' behaviours.
/// This must be implemented along with `NodeAbstract` to create a new node.
pub trait Node: NodeAbstract {
    
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
