use anyhow::Error;
use crate::document::DocumentHandle;
use crate::node_id::NodeId;
use crate::traits::css_system::HasCssSystem;
use crate::traits::document::query::Query;
use crate::traits::node::Node;

pub mod query;

pub trait HasDocument: Sized + HasCssSystem {
    type Document: Document<Self, Node = Self::Node>;
    type Node: Node;
}

pub trait Document<C: HasCssSystem + HasDocument>: Sized {
    type Node: Node;
    type Query: Query;

    fn new(url: &str) -> Self;
    fn register_node_at(&mut self, node: Self::Node, parent_id: NodeId, position: Option<usize>) -> NodeId;

    fn get_handle(&self) -> DocumentHandle<C>;
    fn set_handle(&mut self, handle: DocumentHandle<C>);

    fn get_root_node(&self) -> Option<&Self::Node>;
    fn get_node(&self, id: NodeId) -> Option<&Self::Node>;

    fn stylesheets(&self) -> &Vec<C::CssStylesheet>;
    fn add_stylesheet(&mut self, stylesheet: C::CssStylesheet);

    fn detach_node(&mut self, id: NodeId) -> Option<Self::Node>;
    fn update_node(&mut self, id: NodeId, node: Self::Node);

    fn get_url(&self) -> &str;
    fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Self::Node>;
    fn get_node_clone(&self, id: NodeId) -> Option<Self::Node>;
    fn get_node_by_element_id(&self, name: &str) -> Option<NodeId>;

    fn query(&self, query: &Self::Query) -> Result<Vec<NodeId>, Error>;
}