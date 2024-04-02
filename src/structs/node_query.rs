use crate::traits::node::DynNode;


/// A Node Query either contains a reference to a dynamic Node object, or it is empty. It is the
/// programmer's responsibility to handle cases where the pointer is empty or of the wrong node
/// type.
#[derive(Clone)]
pub enum NodeQuery {
    Some(DynNode),
    None
}

impl NodeQuery {
    
    /// Attempts to unwrap the NodeQuery.
    /// # Panics
    /// Panics if the NodeQuery is empty.
    pub fn unwrap(self) -> DynNode {
        match self {
            NodeQuery::Some(rtr) => rtr,
            NodeQuery::None      => panic!("Attempted to unwrap an empty NodeQuery!")
        }
    }

    /// Returns if this NodeQuery is Some.
    pub fn is_some(&self) -> bool {
        match self {
            NodeQuery::Some(_) => true,
            NodeQuery::None    => false
        }
    }

    /// Returns if this NodeQuery is None.
    pub fn is_none(&self) -> bool {
        match self {
            NodeQuery::Some(_) => false,
            NodeQuery::None    => true
        }
    }
}
