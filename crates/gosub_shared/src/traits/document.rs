use crate::node_id::NodeId;
use crate::traits::css_system::HasCssSystem;
use crate::traits::node::Node;

pub trait HasDocument: Sized + HasCssSystem {
    type Document: Document<Self, Node = Self::Node>;
    type Node: Node;
}

pub trait Document<C: HasCssSystem>: Sized {
    type Node: Node;

    fn new(url: &str) -> Self;
    fn register_node_at(&mut self, node: Self::Node, parent_id: NodeId, position: Option<usize>) -> NodeId;

    fn get_root_node(&self) -> Option<&Self::Node>;
    fn get_node(&self, id: NodeId) -> Option<&Self::Node>;

    fn stylesheets(&self) -> &Vec<C::CssStylesheet>;
    fn add_stylesheet(&mut self, stylesheet: C::CssStylesheet);

    fn detach_node(&mut self, id: NodeId) -> Option<Self::Node>;
    fn attach_node(&mut self, id: NodeId, node: Self::Node);
    fn update_node(&mut self, id: NodeId, node: Self::Node);

    fn get_url(&self) -> &str;
    fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Self::Node>;
    fn get_node_clone(&self, id: NodeId) -> Option<Self::Node>;
}