#![cfg(feature = "glam")]

use std::path::Path;
use std::fs;

use node_tree::prelude::*;
use glam::*;


class! {
    dec Node3D;
    
    let direction: Vec3     = Vec3::ZERO;
    let transform: DAffine3 = DAffine3::IDENTITY;
}


#[test]
fn test_glam() {
    
    // Create a scene and save it.
    let scene: NodeScene = scene! {
        Node3D
    };
    scene.save(Path::new(""), "glam_integration").unwrap();
    
    // Load the scene.
    let scene_loaded: NodeScene = NodeScene::load(Path::new("glam_integration.scn")).unwrap();
    fs::remove_file(Path::new("glam_integration.scn")).unwrap();
}
