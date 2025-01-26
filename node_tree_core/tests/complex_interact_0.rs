use node_tree::prelude::*;
use node_tree::trees::TreeSimple;


class! {
    dec NodeA;

    export default let path_to_target: NodePath;

    hk ready(&mut self) {
        let ptr: Tp<NodeC> = self.get_node((*self.path_to_target).clone()).unwrap();
            ptr.call_this();
    }
}


class! {
    dec NodeB;
}

class! {
    dec NodeC;

    fn call_this(&self) {
        info!(self, "Finished Successfully!");
        self.tree_mut().unwrap().queue_termination();
    }
}


const CONFIG: &str = "
    [Root_0]
    metadata       = { type_name = \"complex_interact_0::NodeA\", is_owner = true }
    path_to_target = \"Node0/NodeB/NodeTarget\"

    [Node0_1]
    metadata = { type_name = \"complex_interact_0::NodeB\", is_owner = false, parent = 0 }

    [Node1_2]
    metadata = { type_name = \"complex_interact_0::NodeB\", is_owner = false, parent = 0 }

    [Node2_3]
    metadata = { type_name = \"complex_interact_0::NodeB\", is_owner = false, parent = 0 }

    [NodeA_4]
    metadata = { type_name = \"complex_interact_0::NodeB\", is_owner = false, parent = 1 }

    [NodeB_5]
    metadata = { type_name = \"complex_interact_0::NodeB\", is_owner = false, parent = 1 }

    [NodeC_6]
    metadata = { type_name = \"complex_interact_0::NodeB\", is_owner = false, parent = 1 }

    [NodeTarget_7]
    metadata = { type_name = \"complex_interact_0::NodeC\", is_owner = false, parent = 5 }
";

#[test]
fn test_complex_interact_0() {
    
    // Attempt to load a scene purely from a config file.
    let scene: NodeScene = NodeScene::load_from_str(CONFIG).unwrap();

    // Create a scene and simulate it.
    let mut tree: Box<TreeSimple> = TreeSimple::new(scene, LoggerVerbosity::All);
    while tree.process().is_active() {}
}
