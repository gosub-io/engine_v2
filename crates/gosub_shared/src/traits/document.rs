use crate::node_id::NodeId;
use crate::traits::css_system::HasCssSystem;
use crate::traits::node::Node;

pub trait HasDocument: Sized + HasCssSystem {
    type Document: Document<Self, Node = Self::Node>;
    type Node: Node;
    // type DocumentBuilder: DocumentBuilder<Self>;
}

pub trait Document<C: HasCssSystem>: Sized {

    type Node: Node;

    fn new(url: &str) -> Self;
    fn register_node_at(&mut self, node: Self::Node, parent_id: NodeId, position: Option<usize>) -> NodeId;

    fn get_root_node(&self) -> Option<&Self::Node>;
    fn get_node(&self, id: NodeId) -> Option<&Self::Node>;
    fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Self::Node>;

    fn do_doc_things_with_css(&self, css: C::CssSystem);
    fn do_document_things(&self);
    fn get_url(&self) -> &str;
}


// pub trait DocumentBuilder<C: HasDocument> {
//     fn new_document(&self, url: &str) -> DocumentHandle<C>;
//     fn new_document_fragment(&self, context_node: &C::Node) -> DocumentHandle<C>;
// }