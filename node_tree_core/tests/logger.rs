use node_tree::structs::logger::{ Logger, SystemCall };
use node_tree::trees::tree_simple::TreeSimple;
use node_tree::prelude::*;


#[test]
pub fn test_logger_bare() -> () {
    let mut logger: Logger = Logger::new(LoggerVerbosity::All);
            logger.post_manual(SystemCall::NodePath("../Grandparent/Parent/NodeA".to_string()), Log::Info("System A Initialized!"));
            logger.post_manual(SystemCall::NodePath("../Grandparent/Parent/NodeB".to_string()), Log::Warn("Some issue occurred! (Simulated Warning)"));
            logger.post_manual(SystemCall::NodePath("../Grandparent/Parent/NodeC".to_string()), Log::Panic("Some crash occured! (Simulated Crash)"));
    
    assert_eq!(logger.to_str().split("\n").collect::<Vec<_>>().len(), 5);
}

#[test]
pub fn test_logger_tree() -> () {
    
    // Enable backtrace.
    std::env::set_var("RUST_BACKTRACE", "1");
    
    // Create the tree.
    let     root: LoggerNode      = LoggerNode::new("Root".to_string());
    let mut tree: Box<TreeSimple> = TreeSimple::new(root, LoggerVerbosity::NoDebug);
    
    while !tree.process().has_terminated() {}
}


class! {
    dec LoggerNode;

    let given_name: String;

    hk _init(given_name: String) {}

    hk ready(&mut self) {
        let name: &str = &self.given_name.clone();
        self.set_name(name);

        if self.depth() < 3 {
            let new_depth: usize = self.depth() + 1;
            
            self.add_child(LoggerNode::new(format!("{}_Node", new_depth)));
            self.add_child(LoggerNode::new(format!("{}_Node", new_depth)));
            self.add_child(LoggerNode::new(format!("{}_Node", new_depth)));
        }
    }

    hk process(&mut self, _delta: f32) {
        if self.depth() != 3 {
            return;
        }

        let grandparent_name: String = {
            let parent:      Tp<LoggerNode> = self.parent().unwrap();
            let grandparent: Tp<LoggerNode> = parent.parent().unwrap();
            
            grandparent.name().to_string()
        };

        if self.name() == "3_Node2" && &grandparent_name == "1_Node" {
            warn!(self, "Simulating warning!");
        }

        if self.name() == "3_Node2" && &grandparent_name == "1_Node2"{
            error!(self, "Simulating panic!");
        }
    }
}
