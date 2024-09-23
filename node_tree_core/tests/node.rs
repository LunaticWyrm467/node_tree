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
        NodeA("Root", 1) {
            NodeA("1_Node", 2) {
                NodeA("2_Node", 3) {
                    NodeA("3_Node", 4),
                    NodeA("3_Node", 5),
                    NodeA("3_Node", 6)
                },
                NodeA("2_Node", 7) {
                    NodeA("3_Node", 8),
                    NodeA("3_Node", 9),
                    NodeA("3_Node", 10)
                },
                NodeA("2_Node", 11) {
                    NodeA("3_Node", 12),
                    NodeA("3_Node", 13),
                    NodeA("3_Node", 14)
                }
            },
            NodeA("1_Node", 15) {
                NodeA("2_Node", 16) {
                    NodeA("3_Node", 17),
                    NodeA("3_Node", 18),
                    NodeA("3_Node", 19)
                },
                NodeA("2_Node", 20) {
                    NodeA("3_Node", 21),
                    NodeA("3_Node", 22),
                    NodeA("3_Node", 23)
                },
                NodeA("2_Node", 24) {
                    NodeA("3_Node", 25),
                    NodeA("3_Node", 26),
                    NodeA("3_Node", 27)
                }
            },
            NodeA("1_Node", 28) {
                NodeA("2_Node", 29) {
                    NodeA("3_Node", 30),
                    NodeA("3_Node", 31),
                    NodeA("3_Node", 32)
                },
                NodeA("2_Node", 33) {
                    NodeA("3_Node", 34),
                    NodeA("3_Node", 35),
                    NodeA("3_Node", 36)
                },
                NodeA("2_Node", 37) {
                    NodeA("3_Node", 38),
                    NodeA("3_Node", 39),
                    NodeA("3_Node", 40)
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
    fn new(name: &str, _example_arg: u8) -> Self {
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
