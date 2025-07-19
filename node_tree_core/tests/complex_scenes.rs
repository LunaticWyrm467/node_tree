use node_tree::prelude::*;
use node_tree::trees::TreeSimple;


class! {
    declare NodeA;
}

class! {
    declare NodeB;
}

class! {
    declare NodeC;
}

class! {
    declare NodeD;

    hk ready(&mut self) {
        panic!("Failed successfully!")
    }
}


#[test]
#[should_panic]
fn test_complex_scenes() {
    let child_scene: NodeScene = scene! {
        NodeA {
            NodeB {
                NodeC,
                NodeD
            }
        }
    };

    let complex_scene: NodeScene = scene! {
        NodeA {
            NodeB {
                $child_scene
            },
            NodeC,
            $child_scene
        }
    };

    let mut tree: Box<TreeSimple> = TreeSimple::new(complex_scene, LoggerVerbosity::All);
    while tree.process().is_active() {}

}
