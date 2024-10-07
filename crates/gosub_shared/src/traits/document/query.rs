use anyhow::Error;
use crate::document::DocumentHandle;
use crate::node_id::NodeId;
use crate::traits::document::HasDocument;

pub trait Condition {
    fn equals_tag(tag_name: &str) -> Self;
    fn equals_id(id: &str) -> Self;
    fn contains_class(class: &str) -> Self;
    fn contains_attribute(attribute: &str) -> Self;
    fn contains_child_tag(child_tag: &str) -> Self;
    fn has_parent_tag(parent_tag: &str) -> Self;
}


pub trait SearchType: PartialEq {
    fn uninitialized() -> Self;
    fn find_first() -> Self;
    fn find_all() -> Self;
}

// Query is a trait that represents a query that can be performed on a document.
pub trait Query {
    type SearchType: SearchType;
    type Condition: Condition;

    fn new(search_type: Self::SearchType, conditions: Vec<Self::Condition>) -> Self;
    fn search_type(&self) -> Self::SearchType;
    fn conditions(&self) -> Vec<Self::Condition>;
}

// QueryProcessor is a trait that represents a query processor that can be used to process queries.
pub trait QueryProcessor<C: HasDocument>: Sized {
    type Query: Query;

    fn query(&self, handle: DocumentHandle<C>, query: &Self::Query) -> Result<Vec<NodeId>, Error>;
}