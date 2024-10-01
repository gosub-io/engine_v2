use crate::node_id::NodeId;
use crate::traits::css_system::HasCssSystem;
use crate::traits::node::HasNode;

pub trait HasDocument: Sized + HasCssSystem + HasNode {
    type Document: Document<Self::CssSystem, Self::Node>;
}

pub trait Document<C: HasCssSystem, N: HasNode>: Sized + HasDocument {
    fn new(url: &str) -> Self;
    fn register_node_at(&mut self, node: Self::Node, parent_id: NodeId, position: Option<usize>) -> NodeId;

    fn get_root_node(&self) -> Option<&Self::Node>;
    fn get_node(&self, id: NodeId) -> Option<&Self::Node>;
    fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Self::Node>;

    fn do_doc_things_with_css(&self, css: Self::CssSystem);
    fn do_document_things(&self);
}
