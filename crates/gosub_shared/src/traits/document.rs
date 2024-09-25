use crate::node_id::NodeId;
use crate::traits::css_system::{CssSystem};
use crate::traits::node::HasNode;

pub trait HasDocument: Sized {
    type CssSystem: CssSystem;
    type Document: Document<Self>;
}

// impl<HD: HasDocument> HasCssSystem for HD {
//     type CssSystem = HD::CssSystem;
// }

pub trait Document<C: HasNode>: Sized {
    fn new(url: &str) -> Self;
    fn register_node_at(&mut self, node: C::Node, parent_id: NodeId, position: Option<usize>) -> NodeId;

    fn get_root_node(&self) -> Option<&C::Node>;
    fn get_node(&self, id: NodeId) -> Option<&C::Node>;
    fn get_node_mut(&mut self, id: NodeId) -> Option<&mut C::Node>;
}
