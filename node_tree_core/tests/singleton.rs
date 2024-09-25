use node_tree::prelude::*;
use node_tree::trees::tree_simple::TreeSimple;


#[derive(Debug, Clone, Abstract)]
pub struct Singleton {
    base: NodeBase
}

impl Singleton {
    fn new() -> Self {
        Singleton {
            base: NodeBase::new("SomeSingletonNode".to_string())
        }
    }
}

impl Node for Singleton {
    fn ready(&mut self) {
        let status: bool = self.register_as_singleton("TheOneAndOnly".to_string());
        assert_eq!(status, true);
        self.tree_mut().unwrap().terminate();
    }
}


#[test]
fn test_singleton_registration() {
    let     singleton: Singleton       = Singleton::new();
    let mut tree:      Box<TreeSimple> = TreeSimple::new(singleton, LoggerVerbosity::NoDebug);

    while !tree.process().has_terminated() {}
}
