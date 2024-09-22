use node_tree::trees::tree_simple::TreeSimple;
use node_tree::prelude::*;


#[test]
fn test_node_integration() {
    
    // Enable backtrace.
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    // Initialize the NodeScene.
    let scene: NodeScene = scene! {
        NodeA("Root") {
            NodeA("1_Node") {
                NodeA("2_Node") {
                    NodeA("3_Node"),
                    NodeA("3_Node"),
                    NodeA("3_Node")
                },
                NodeA("2_Node") {
                    NodeA("3_Node"),
                    NodeA("3_Node"),
                    NodeA("3_Node")
                },
                NodeA("2_Node") {
                    NodeA("3_Node"),
                    NodeA("3_Node"),
                    NodeA("3_Node")
                }
            },
            NodeA("1_Node") {
                NodeA("2_Node") {
                    NodeA("3_Node"),
                    NodeA("3_Node"),
                    NodeA("3_Node")
                },
                NodeA("2_Node") {
                    NodeA("3_Node"),
                    NodeA("3_Node"),
                    NodeA("3_Node")
                },
                NodeA("2_Node") {
                    NodeA("3_Node"),
                    NodeA("3_Node"),
                    NodeA("3_Node")
                }
            },
            NodeA("1_Node") {
                NodeA("2_Node") {
                    NodeA("3_Node"),
                    NodeA("3_Node"),
                    NodeA("3_Node")
                },
                NodeA("2_Node") {
                    NodeA("3_Node"),
                    NodeA("3_Node"),
                    NodeA("3_Node")
                },
                NodeA("2_Node") {
                    NodeA("3_Node"),
                    NodeA("3_Node"),
                    NodeA("3_Node")
                }
            }
        }
    };

    // Create the tree.
    let mut tree: Box<TreeSimple> = TreeSimple::new(scene, LoggerVerbosity::NoDebug);

    // Begin operations on the tree.
    tree.start();
    loop {
        if tree.process().has_terminated() {
            break;
        }
    }
}


#[derive(Debug, Clone, Abstract)]
pub struct NodeA {
    base: NodeBase
}

impl NodeA {
    fn new(name: &str) -> Self {
        NodeA { base: NodeBase::new(name.to_string()) }
    }
}

impl Node for NodeA {
    fn ready(&mut self) -> () {
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
