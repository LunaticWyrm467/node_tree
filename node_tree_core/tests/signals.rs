use node_tree::prelude::*;
use node_tree::trees::TreeSimple;


class! {
    dec NodeA;
    
    sig on_event(count: u8);
    
    let count: u8 = 0;
    
    hk ready(&mut self) {
        let child: Tp<NodeB> = self.get_child(0).unwrap();
        connect! { on_event -> child.listener };
    }
    
    hk process(&mut self, _delta: f32) {
        self.on_event.emit(self.count);
        self.count += 1;
    }
}


class! {
    dec NodeB;

    fn listener(&self, count: &u8) {
        if *count == 3 {
            panic!("This was successful!");
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
