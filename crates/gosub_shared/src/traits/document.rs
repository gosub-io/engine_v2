use anyhow::Error;
use crate::document::DocumentHandle;
use crate::node_id::NodeId;
use crate::traits::css_system::HasCssSystem;
use crate::traits::node::Node;

// QueryCondition is a trait that represents a condition that can be used in a query.
pub trait Condition {}

// SearchType is a trait that represents the type of search that is being performed.
pub trait SearchType {}

// Query is a trait that represents a query that can be performed on a document.
pub trait Query {
    fn search_type(&self) -> impl SearchType;
    fn conditions(&self) -> Vec<impl Condition>;
}

// QueryProcessor is a trait that represents a query processor that can be used to process queries.
pub trait QueryProcessor<C: HasDocument>: Sized {
    fn query(handle: DocumentHandle<C>, query: &impl Query) -> Result<Vec<NodeId>, Error>;
}

pub trait HasDocument: Sized + HasCssSystem {
    type Document: Document<Self, Node = Self::Node>;
    type Node: Node;
    type QueryProcessor: QueryProcessor<Self>;
}

pub trait Document<C: HasCssSystem + HasDocument>: Sized {
    type Node: Node;
    type QueryProcessor: QueryProcessor<C>;

    fn new(url: &str) -> Self;
    fn register_node_at(&mut self, node: Self::Node, parent_id: NodeId, position: Option<usize>) -> NodeId;

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
}