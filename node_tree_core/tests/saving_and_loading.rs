use std::collections::HashMap;

use std::env;
use std::path::Path;
use std::fs;

use node_tree::prelude::*;
use node_tree::trees::TreeSimple;


class! {
    declare NodeA;

    export let field_1: u64    = 0;
    export let field_2: String = "Hello World!".to_string();
    export let field_3: bool   = false;
}

class! {
    declare NodeB;

    export let field_a: u8   = 255;
    export let field_b: char = 'x';
}

class! {
    declare NodeC;

    export let field_0: Vec<u8>           = vec![0, 1];
    export let field_1: HashMap<char, u8> = vec![('a', 0), ('b', 1), ('c', 2)].into_iter().collect();

    hk loaded(&mut self) {
        self.field_0.push(2);
    }

    hk ready(&mut self) {
        assert_eq!(self.field_0.len(), 3); // Assert that loaded has been called.
        self.tree_mut().unwrap().queue_termination();
    }
}


#[test]
fn test_writing_to_disk() {

    // Set this for debugging.
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }
    
    // Create a scene and save it.
    let scene: NodeScene = scene! {
        NodeA [
            NodeB,
            NodeC
        ]
    };
    scene.save(Path::new(""), "foo").unwrap();
    
    // Load the scene.
    let scene_loaded: NodeScene = NodeScene::load(Path::new("foo.scn")).unwrap();
    fs::remove_file(Path::new("foo.scn")).unwrap();

    // Hash the tree structures and verify their integrity.
    assert_eq!(scene.structural_hash(), scene_loaded.structural_hash());
    
    // Simulate the tree.
    let mut tree: Box<TreeSimple> = TreeSimple::new(scene_loaded, LoggerVerbosity::All);
    while tree.process().is_active() {}
}


class! {
    declare Root;

    hk ready(&mut self) {
        
        // Create the exact scene and get it's hash.
        let model_scene: NodeScene = scene! {
            Root [
                NodeA [
                    NodeB,
                    NodeB
                ]
            ]
        };
        let model_hash: u64 = model_scene.structural_hash();
        
        // Save a scene from this node, and get this scene's hash.
        let this_scene: NodeScene = self.save_as_branch();
        let this_hash:  u64       = this_scene.structural_hash();

        // Assert that the hashes are the same.
        assert_eq!(model_hash, this_hash);
    }

    hk process(&mut self, _delta: f32) {
        self.tree_mut().unwrap().queue_termination();
    }
}

#[test]
fn test_branch_saving() {
    
    // Create a scene and simulate it.
    let scene: NodeScene = scene! {
        Root [
            NodeA [
                NodeB,
                NodeB
            ]
        ]
    };

    let mut tree: Box<TreeSimple> = TreeSimple::new(scene, LoggerVerbosity::All);
    while tree.process().is_active() {}
}
