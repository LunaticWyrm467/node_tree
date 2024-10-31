use node_tree::prelude::*;
use node_tree::trees::tree_simple::TreeSimple;


class! {
    dec NodeA;

    hk ready(&mut self) {
        if self.depth() == 2 && self.name() == "NodeA1" {
            self.post(Log::Warn("Failed to Initialize!"));
        }
        
        if self.depth() == 1 && self.name() == "NodeA" {
            self.get_node::<NodeA>(NodePath::from_str("Foo/Bar")).unwrap();
        }
    }
}


#[test]
#[should_panic]
fn test_tree_pointer() {
    let scene: NodeScene = scene! {
        NodeA {
            NodeA,
            NodeA,
            NodeA {
                NodeA,
                NodeA,
                NodeA
            }
        }
    };

    let mut tree: Box<TreeSimple> = TreeSimple::new(scene, LoggerVerbosity::All);
    while !tree.process().has_terminated() {}
}
