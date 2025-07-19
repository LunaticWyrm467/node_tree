use node_tree::prelude::*;
use node_tree::trees::TreeSimple;


class! {
    declare NodeA;
    
    signal on_event(count: u8);
    
    default let count: u8;
    
    hk ready(&mut self) {
        let child: Tp<NodeB> = self.get_child(0).unwrap();
        let this:  Tp<NodeA> = self.this();

        connect! { on_event -> child.listener };
        connect! { on_event -> this.listener  };
    }
    
    hk process(&mut self, _delta: f32) {
        self.on_event.emit(self.count);
        self.count += 1;
    }

    fn listener(&self, count: &u8) {
        warn!(self, "Count is {count}");
    }
}


class! {
    declare NodeB;

    fn listener(&self, count: &u8) {
        if *count == 3 {
            error!(self, "This was successful!");
            panic!();
        }
    }
}


#[test]
#[should_panic]
fn test_signals() {
    let scene: NodeScene = scene! {
        NodeA {
            NodeB
        }
    };

    let mut tree: Box<TreeSimple> = TreeSimple::new(scene, LoggerVerbosity::All);
    while tree.process().is_active() {}
}
