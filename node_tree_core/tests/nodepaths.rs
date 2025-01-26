use std::collections::HashMap;

use node_tree::prelude::*;
use node_tree::trees::TreeSimple;


class! {
    dec Unit;
}

class! {
    dec Crawler;

    hk process(&mut self, _delta: f32) {
        let paths_and_targets: HashMap<NodePath, &str> = vec![
            (nodepath!("Child"),            "Child"),
            (nodepath!("Child/Grandchild"), "Grandchild"),
            (nodepath!("."),                "Self"),
            (nodepath!(".."),               "Parent"),
            (nodepath!("../Sibling"),       "Sibling"),
            (nodepath!("../.."),            "Grandparent"),
            (nodepath!("/Grandparent"),     "Grandparent")
        ].into_iter().collect();
        
        for (path, target) in paths_and_targets {
            if target != "Self" {
                assert_eq!(self.get_node::<Unit>(path).unwrap().name(), target);
            } else {
                assert_eq!(self.get_node::<Crawler>(path).unwrap().name(), target);
            }
        }
        self.tree_mut()
            .unwrap()
            .queue_termination();
    }
}


#[test]
fn test_nodepaths() {
    let scene: NodeScene = scene! {
        Unit: "Grandparent" {
            Unit: "Parent" {
                Crawler: "Self" {
                    Unit: "Child" {
                        Unit: "Grandchild"
                    }
                },
                Unit: "Sibling"
            }
        }
    };

    let mut tree: Box<TreeSimple> = TreeSimple::new(scene, LoggerVerbosity::All);
    while tree.process().is_active() {}
}
