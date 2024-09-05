use node_tree::prelude::*;


#[test]
fn test_node_integration() {
    
    // Enable backtrace.
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    // Create the tree.
    let     root: NodeA         = NodeA::new("Root".to_string());
    let mut tree: Box<NodeTree> = NodeTree::new(root, LoggerVerbosity::NoDebug);

    // Begin operations on the tree.
    tree.start();
    tree.process();
}


#[derive(Debug, Abstract)]
pub struct NodeA {
    base: NodeBase
}

impl NodeA {
    fn new(name: String) -> Self {
        NodeA { base: NodeBase::new(name) }
    }
}

impl Node for NodeA {
    fn ready(&mut self) -> () {
        if self.depth() < 3 {
            let depth_new: usize = self.depth() + 1;

            self.add_child(NodeA::new(format!("{}_Node", depth_new)));
            self.add_child(NodeA::new(format!("{}_Node", depth_new)));
            self.add_child(NodeA::new(format!("{}_Node", depth_new)));
        }
        if self.is_root() {
            println!("{:?}", self.children());
        }
    }

    fn process(&mut self, delta: f32) -> () {
        println!("{} | {}", self.name(), 1f32 / delta);
        if self.is_root() {
            match self.get_node::<NodeA>(NodePath::from_str("1_Node/2_Node1/3_Node2")) {
                Some(node) => println!("{:?}", node),
                None       => ()
            }
        }

        if self.children().is_empty() {
            self.free();   // We test the progressive destruction of nodes from the tip of the tree
                           // to the base.
        }
    }
}
