use crate::traits::{ node::Node, instanceable::Instanceable };


/*
 * Node Scene
 *      Struct
 */


/// A recursive structure that allows for the storage of a dormant scene of nodes.
/// The root node is what every node in the scene will have its owner set to.
#[derive(Debug)]
pub struct NodeScene {
    this:     *mut dyn Node,
    children: Vec<NodeScene>
}

impl NodeScene {
    
    /// Creates a new `NodeScene` with a root node.
    pub fn new<N: Node>(root: N) -> Self {
        NodeScene {
            this:     Box::into_raw(root.to_dyn_box()),
            children: Vec::new()
        }
    }

    /// Appends a `NodeScene` as a child.
    pub fn append(&mut self, child: NodeScene) {
        self.children.push(child);
    }

    /// Returns this `NodeScene` instance's associated node.
    /// 
    /// # Safety
    /// This is marked unsafe as if the resulting `Box<T>` is dropped, the internal pointer could
    /// be invalidated.
    pub unsafe fn get_node(&self) -> Box<dyn Node> {
        Box::from_raw(self.this)
    }

    /// Gets the children.
    pub fn children(&self) -> &[NodeScene] {
        &self.children
    }
}

impl Clone for NodeScene {
    fn clone(&self) -> Self {
        let cloned_node = unsafe {
            let node_original: Box<dyn Node> = Box::from_raw(self.this);
            let node_new:      Box<dyn Node> = node_original.clone_as_instance();

            Box::into_raw(node_original); // Convert the box back so that its instance isn't deallocated when dropped.
            Box::into_raw(node_new)
        };

        // Recursively clone children
        let cloned_children: Vec<NodeScene> = self.children.iter().map(|child| child.clone()).collect();
        NodeScene {
            this:     cloned_node,
            children: cloned_children,
        }
    }
}

impl Instanceable for NodeScene {
    fn iterate<F: FnMut(Option<*mut dyn Node>, *mut dyn Node)>(self, mut iterator: F) {
        iterator(None, self.this);

        // Recursive function to traverse the tree
        fn traverse<F: FnMut(Option<*mut dyn Node>, *mut dyn Node)>(
            node:     NodeScene,
            parent:   *mut dyn Node,
            iterator: &mut F
        ) {
            for child in node.children {

                // Call the iterator for the child node
                iterator(Some(parent), child.this);

                // Recursively traverse the child's children
                let child_this: *mut dyn Node = child.this;
                traverse(child, child_this, iterator);
            }
        }

        // Start the traversal from the root.
        let self_this: *mut dyn Node = self.this;
        traverse(self, self_this, &mut iterator);
    }
}
