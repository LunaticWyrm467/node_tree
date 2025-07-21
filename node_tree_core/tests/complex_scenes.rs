use node_tree::prelude::*;
use node_tree::trees::TreeSimple;


class! {
    declare NodeA;
}

class! {
    declare NodeB;

    pub let field1: String = "value1".to_string();
    pub let field2: i32 = 2;

    hk process(&mut self, _: f32) {
        assert_ne!(*self.field1, "value1");
        assert_ne!(*self.field2, 2);
    }
}

class! {
    declare NodeC;
}

mod private {
    use node_tree::prelude::class;

    class! {
        pub declare NodeD;

        hk ready(&mut self) {
            panic!("Failed successfully!")
        }
    }
}


#[test]
#[should_panic]
fn test_complex_scenes() {
    let child_scene: NodeScene = scene! {
        NodeA [
            NodeB {
                field1: "foo".to_string(),
                field2: 42,
                [
                    NodeC,
                    private::NodeD // Paths are supported!
                ]
            }
        ]
    };

    let complex_scene: NodeScene = scene! {
        NodeA [
            NodeB {
                field1: "bar".to_string(),
                field2: 84
            },
            NodeC [$child_scene],
            $child_scene
        ]
    };

    let mut tree: Box<TreeSimple> = TreeSimple::new(complex_scene, LoggerVerbosity::All);
    while tree.process().is_active() {}

}
