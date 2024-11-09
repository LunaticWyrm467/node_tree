use node_tree::trees::tree_simple::TreeSimple;
use node_tree::prelude::*;

#[test]
fn test_node_integration() {
    
    // Enable backtrace.
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    // Initialize the NodeScene.
    let child_scene: NodeScene = scene! {
        NodeA("2_Node", 3) {
            NodeA("3_Node", 4),
            NodeA("3_Node", 5),
            NodeA("3_Node", 6)
        }
    };
    let parent_scene: NodeScene = scene! {
        NodeA("1_Node", 2) {
            $child_scene,
            $child_scene,
            $child_scene,
        }
    };
    let scene: NodeScene = scene! {
        NodeA("Root", 1) {
            $parent_scene,
            $parent_scene,
            $parent_scene,
        }
    };

    // Create the tree.
    let mut tree: Box<TreeSimple> = TreeSimple::new(scene, LoggerVerbosity::NoDebug);
    while !tree.process().has_terminated() {}
}


class! {
    dec NodeA;

    let  given_name:  String;
    let _example_arg: u8;

    hk _init(name: &str, _example_arg: u8) {
        let given_name: String = name.to_string();
    }

    hk ready(&mut self) {
        let name: &str = &self.given_name.clone();
        self.set_name(name);

        if self.is_root() {
            println!("{:?}", self.children());
        }
    }

    hk process(&mut self, delta: f32) {
        println!("{} | {}", self.name(), 1f32 / delta);
        if self.is_root() {
            match self.get_node::<NodeA>(NodePath::from_str("1_Node/2_Node1/3_Node2")).to_option() {
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
