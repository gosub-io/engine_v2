use crate::node::node_data::comment::CommentData;
use crate::node::node_data::doctype::DocTypeData;
use crate::node::node_data::document::DocumentData;
use crate::node::node_data::element::ElementData;
use crate::node::node_data::text::TextData;

pub mod comment;
pub mod doctype;
pub mod document;
pub mod element;
pub mod text;

#[derive(Debug, Clone)]
pub enum NodeData {
    Element(ElementData),
    Text(TextData),
    Comment(CommentData),
    DocType(DocTypeData),
    Document(DocumentData),
}

impl gosub_shared::traits::node::NodeData for NodeData {}

impl From<ElementData> for NodeData {
    fn from(data: ElementData) -> Self {
        NodeData::Element(data)
    }
}
impl From<TextData> for NodeData {
    fn from(data: TextData) -> Self {
        NodeData::Text(data)
    }
}
impl From<CommentData> for NodeData {
    fn from(data: CommentData) -> Self {
        NodeData::Comment(data)
    }
}
impl From<DocTypeData> for NodeData {
    fn from(data: DocTypeData) -> Self {
        NodeData::DocType(data)
    }
}
impl From<DocumentData> for NodeData {
    fn from(data: DocumentData) -> Self {
        NodeData::Document(data)
    }
}
