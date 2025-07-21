use node_tree::prelude::*;
use node_tree::trees::TreeSimple;


class! {
    declare ControlNode;

    hk process(&mut self, _: f32) {
        let mut log: Vec<&'static str> = Vec::with_capacity(3);

        for i in 0..self.num_children() {
            let child:  TpDyn          = self.get_child_dyn(i).unwrap();
            let casted: &dyn Attribute = child.cast().unwrap();

            log.push(casted.say_something());
        }

        assert_eq!(log, vec!["Foo!", "Bar!", "Baz!"]);

        self
            .tree_mut()
            .unwrap()
            .queue_termination();
    }
}

#[test]
fn test_signals() {
    let scene: NodeScene = scene! {
        ControlNode [
            AttributeNode1,
            AttributeNode2,
            AttributeNode3
        ]
    };

    let mut tree: Box<TreeSimple> = TreeSimple::new(scene, LoggerVerbosity::All);
    while tree.process().is_active() {}
}


class! {
    declare AttributeNode1 extends Attribute;
}

class! {
    declare AttributeNode2 extends Attribute;
}

class! {
    declare AttributeNode3 extends Attribute;
}

trait Attribute {
    fn say_something(&self) -> &'static str;
}

impl Attribute for AttributeNode1 {
    fn say_something(&self) -> &'static str {
        "Foo!"
    }
}

impl Attribute for AttributeNode2 {
    fn say_something(&self) -> &'static str {
        "Bar!"
    }
}

impl Attribute for AttributeNode3 {
    fn say_something(&self) -> &'static str {
        "Baz!"
    }
}