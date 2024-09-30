use gosub_shared::traits::document::Document;
use gosub_shared::traits::node::{CommentData, DocTypeData, ElementData, TextData, HasNode};
use gosub_shared::traits::node::Node;

pub struct NodeBuilder<C: HasNode> {
    _marker: std::marker::PhantomData<C>,
}

impl<C: HasNode> HasNode for NodeBuilder<C> {
    type Node = C::Node;
}

impl<C: HasNode> NodeBuilder<C> {
    pub fn new_element_node(name: &str, namespace: &str) -> C::Node {
        C::Node::new(C::Node::NodeData::Element(ElementData::new(name.into(), namespace.into())))
    }

    pub fn new_text_node(content: &str) -> C::Node {
        Self::Node::new(C::Node::NodeData::Text(TextData::new(content.into())))
    }

    pub fn new_comment_node(content: &str) -> C::Node {
        Self::Node::new(C::Node::NodeData::Comment(CommentData::new(content.into())))
    }

    pub fn new_doctype_node(name: &str, public_id: &str, system_id: &str) -> C::Node {
        Self::Node::new(C::Node::NodeData::DocType(DocTypeData::new(name.into(), public_id.into(), system_id.into())))
    }
}