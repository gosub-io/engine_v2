use crate::node_id::NodeId;
use crate::traits::css_system::HasCssSystem;
use crate::traits::node::{HasNode, Node, NodeBuilder};

pub trait HasDocument: Sized + HasCssSystem {
    type Document: Document<Self::CssSystem>;
}



// impl<HD: HasDocument> HasCssSystem for HD {
//     type CssSystem = HD::CssSystem;
// }

pub trait Document<C: HasCssSystem>: Sized + HasDocument {
    
    type Node: Node;
    type NodeBuilder: NodeBuilder<Self::Node>;

    fn new(url: &str) -> Self;
    fn register_node_at(&mut self, node: Self::Node, parent_id: NodeId, position: Option<usize>) -> NodeId;

    fn get_root_node(&self) -> Option<&Self::Node>;
    fn get_node(&self, id: NodeId) -> Option<&Self::Node>;
    fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Self::Node>;
    
    fn do_document_things(&self) {
        println!("Doing document things");
    }
}
