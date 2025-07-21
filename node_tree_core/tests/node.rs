use node_tree::trees::TreeSimple;
use node_tree::prelude::*;

#[test]
fn test_node_integration() {
    
    // Enable backtrace.
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    // Initialize the NodeScene.
    let child_scene: NodeScene = scene! {
        NodeA(3): "2_Node" [
            NodeA(4): "3_Node",
            NodeA(5): "3_Node",
            NodeA(6): "3_Node"
        ]
    };
    let parent_scene: NodeScene = scene! {
        NodeA(2): "1_Node" [
            $child_scene,
            $child_scene,
            $child_scene,
        ]
    };
    let scene: NodeScene = scene! {
        NodeA(1): "Root" [
            $parent_scene,
            $parent_scene,
            $parent_scene,
        ]
    };

    // Create the tree.
    let mut tree: Box<TreeSimple> = TreeSimple::new(scene, LoggerVerbosity::NoDebug);
    while !tree.process().has_terminated() {}
}


class! {
    declare NodeA;

    let _example_arg: u8;

    hk _init(_example_arg: u8) {}

    hk ready(&mut self) {
        if self.is_root() {
            println!("{:?}", self.children());
        }
    }

    hk process(&mut self, delta: f32) {
        println!("{} | {}", self.name(), 1f32 / delta);
        if self.is_root() {
            match self.get_node::<NodeA>(nodepath!("1_Node/2_Node1/3_Node2")).to_option() {
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
