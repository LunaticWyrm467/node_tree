# NodeTree
![Crates.io License](https://img.shields.io/crates/l/node_tree?color=green)
[![Crates.io Version](https://img.shields.io/crates/v/node_tree)](https://crates.io/crates/node_tree)
[![Documentation](https://docs.rs/node_tree/badge.svg)](https://docs.rs/node_tree)

**NodeTree** is a framework to create large scalable programs and games through a tree of processes. Each process is fully autonomous and is capable of storing its own state or data, and communicating with other processes. These processes are known as Nodes.

**⚠️WARNING⚠️**<br>
THIS IS A NIGHTLY-DEPENDENT CRATE.<br>
This crate is in early development. Beware of possible bugs or safety violations.<br>
The specific nightly version this crate uses is v1.78.

## Getting Started!
Simply either run `cargo add node_tree` at the terminal directed towards the directory of your project, or add `node_tree = X.X` to your `cargo.toml` file.

To begin creating a program in Rust that utilizes a `NodeTree`, we must first create a root `Node`. In order to reduce boilerplate, we will use the included `NodeSys` derive macro to implement the required `Dynamic` and `NodeAbstract` traits. We will then implement the `Node` trait ourselves.
```rust
#![feature(arbitrary_self_types)]   // Required for now.
use node_tree::prelude::*;


#[derive(Debug, Clone, NodeSys)]
pub struct NodeA {
    base: Rc<NodeBase>   // Required for Nodes.
}

// To make things simple, it is advised to have most node constructors return the node
// instance wrapped inside of this crate's `Hp<T>` pointer.
impl NodeA {
    fn new(name: String) -> Hp<Self> {
        Hp::new(NodeA { base: NodeBase::new(name) })
    }
}

// Example implementation of the Node trait with custom behaviours.
impl Node for NodeA {

    /// Runs once the Node is added to the NodeTree.
    fn ready(self: Hp<Self>) -> () {

        // To show off how you could add children nodes.
        if self.depth() < 3 {
            self.add_child(NodeA::new(format!("{}_Node", self.depth() + 1)));
            self.add_child(NodeA::new(format!("{}_Node", self.depth() + 1)));
            self.add_child(NodeA::new(format!("{}_Node", self.depth() + 1)));
        }

        if self.is_root() {
            println!("{:#?}", self.children());
        }
    }

    /// Runs once per frame. Provides a delta value in seconds between frames.
    fn process(self: Hp<Self>, delta: f32) -> () {

        // Example of using the delta value to calculate the current framerate.
        println!("{} | {}", self.name(), 1f32 / delta);

        // Using the NodePath, you can reference other nodes in the NodeTree from this node.
        if self.is_root() {
            match self.get_node(NodePath::from_str("1_Node/2_Node1/3_Node2")) {
                Some(node) => println!("{:?}", node),
                None       => ()
            }
        }

        // Nodes can be destroyed. When destroyed, their references from the NodeTree are cleaned up as well.
        // If the root node is destroyed, then the program automatically exits. (There are other ways to
        // terminate the program such as the queue_termination() function on the NodeTree instance).
        if self.children().is_empty() {
            self.free();   // We test the progressive destruction of nodes from the tip of the tree
                           // to the base.
        }
    }

    /// Runs once a Node is removed from the NodeTree, whether that is from the program itself terminating or not.
    fn terminal(self: Hp<Self>) -> () {}   // We do not do anything here for this example.

    /// Returns this node's process mode.
    /// Each process mode controls how the process() function behaves when the NodeTree is paused or not.
    /// (The NodeTree can be paused or unpaused with the pause() or unpause() functions respectively.)
    fn process_mode(self: Hp<Self>) -> ProcessMode {
        ProcessMode::Inherit    // We will return the default value, which inherits the behaviour from
                                // the parent node.
    }
}
```

Finally, in order to activate our `NodeTree`, we must instance the root `Node` and feed it into the `NodeTree` constructor.
```rust
// ...previous implementations

fn main() -> () {

    // Create the tree.
    let root: Hp<NodeA>    = NodeA::new("Root".to_string());
    let tree: Hp<NodeTree> = NodeTree::new(root, LoggerVerbosity::NoDebug);

    // Begin operations on the tree.
    tree.start();
    tree.process();   // This will run an indefinite loop until the program exits.
}
```

Logging is also supported. Here is an example setup with an output of a few warnings and a crash. Note that the crash header/footer are customizable, and that the output is actually colored in a real terminal.
```rust
/// Root Node
#[derive(Debug, Clone, NodeSys)]
pub struct LoggerNode {
    base: Rc<NodeBase>
}

impl LoggerNode {
    fn new(name: String) -> Hp<Self> {
        Hp::new(LoggerNode { base: NodeBase::new(name) })
    }
}

impl Node for LoggerNode {
    fn ready(self: Hp<Self>) -> () {
        if self.depth() < 3 {
            self.add_child(LoggerNode::new(format!("{}_Node", self.depth() + 1)));
            self.add_child(LoggerNode::new(format!("{}_Node", self.depth() + 1)));
            self.add_child(LoggerNode::new(format!("{}_Node", self.depth() + 1)));
        }
    }

    fn process(self: Hp<Self>, _delta: f32) -> () {
        if self.name() == "3_Node2" && self.parent().unwrap().parent().unwrap().name() == "1_Node" {   // In the real world, you should probably have a better way of doing this.
            self.post_to_log(Log::Warn("Simulating warning!"));
        }

        if self.name() == "3_Node2" && self.parent().unwrap().parent().unwrap().name() == "1_Node2"{
            self.post_to_log(Log::Panic("Simulating panic!"));
        }
    }
}
```

```console
<22/04/2024 17:25:46 UTC> | [Root/1_Node/2_Node/3_Node2] | WARN | Simulating warning!
<22/04/2024 17:25:46 UTC> | [Root/1_Node/2_Node1/3_Node2] | WARN | Simulating warning!
<22/04/2024 17:25:46 UTC> | [Root/1_Node/2_Node2/3_Node2] | WARN | Simulating warning!
<22/04/2024 17:25:46 UTC> | [Root/1_Node2/2_Node/3_Node2] | PANIC! | Simulating panic!

Unfortunately the program has crashed. Please contact the development team with the following crash report as well as the attachment of the log posted during the time of the crash.

[REPORT START]
Root
├── 1_Node
│   ├── 2_Node
│   │   ├── 3_Node
│   │   ├── 3_Node1
│   │   └── 3_Node2
│   ├── 2_Node1
│   │   ├── 3_Node
│   │   ├── 3_Node1
│   │   └── 3_Node2
│   └── 2_Node2
│       ├── 3_Node
│       ├── 3_Node1
│       └── 3_Node2
├── 1_Node1
│   ├── 2_Node
│   │   ├── 3_Node
│   │   ├── 3_Node1
│   │   └── 3_Node2
│   ├── 2_Node1
│   │   ├── 3_Node
│   │   ├── 3_Node1
│   │   └── 3_Node2
│   └── 2_Node2
│       ├── 3_Node
│       ├── 3_Node1
│       └── 3_Node2
└── 1_Node2
    ├── 2_Node
    │   ├── 3_Node
    │   ├── 3_Node1
    │   └── 3_Node2
    ├── 2_Node1
    │   ├── 3_Node
    │   ├── 3_Node1
    │   └── 3_Node2
    └── 2_Node2
        ├── 3_Node
        ├── 3_Node1
        └── 3_Node2

[Same-Frame Warnings]
3_Node2 - Simulating warning!
3_Node2 - Simulating warning!
3_Node2 - Simulating warning!

[Same-Frame Panics]
3_Node2 - Simulating panic!

[REPORT END]
Time of Crash: 22/04/2024 17:25:46
Exit Code: 1

Goodbye World! (Program Exited)
```

## Features
- 🏗️ An easy abstraction framework for different processes to communicate and interact with each other in a scalable manner. Inspired by Godot!
- ⏯️ The ability to `pause()` and `unpause()` the `NodeTree`, and fine tune individual `Node` behaviours for when a tree is paused/unpaused.
- 📡 Various methods to communicate with other nodes, such as `owner()`, `parent()`, `get_child()`, `children()`, and `get_node()`.
- 🔗 An abstracted smart pointer known as `Hp<T>` which clones implicitly to reduce syntax noise and allows for low boilerplate.
- 👪 The ability to manage nodes with `add_child()` and `remove_child()`.
- 📝 Includes a dynamic logging system that is deeply integrated with the node framework.
- 🌲 Allows for the direct referencing of the `NodeTree` through a node's `root()` function.
- 📚 TODO: A caching system hosted on the `NodeTree` to act as a safe interface to ensure the `Hp<T>` soundness, and increase performance!
- 📜 TODO: Includes a method to save and handle individual node scenes, such as the handy visual macro `Scene!`.
