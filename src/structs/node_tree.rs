use crate::MutableArc;
use crate::traits::node::Node;

pub struct NodeTree {
    root:   MutableArc<dyn Node>,
    paused: bool
}


#[derive(Debug, Clone)]
pub enum ProcessMode {
    Inherit,
    Always,
    Pausable,
    Inverse,
}
