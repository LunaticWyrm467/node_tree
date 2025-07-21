use node_tree::prelude::*;
use node_tree::trees::tree_simple::TreeSimple;


class! {
    declare NodeA;

    hk ready(&mut self) {
        if self.depth() == 2 && self.name() == "NodeA1" {
            warn!(self, "Failed to Initialize!");
        }
        
        if self.depth() == 1 && self.name() == "NodeA" {
            self.get_node::<NodeA>(nodepath!("Foo/Bar")).unwrap();
        }
    }
}


#[test]
#[should_panic]
fn test_tree_pointer() {
    let scene: NodeScene = scene! {
        NodeA [
            NodeA,
            NodeA,
            NodeA [
                NodeA,
                NodeA,
                NodeA
            ]
        ]
    };

    let mut tree: Box<TreeSimple> = TreeSimple::new(scene, LoggerVerbosity::All);
    while !tree.process().has_terminated() {}
}
