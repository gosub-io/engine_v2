use crate::node::node_data::comment::CommentData;
use crate::node::node_data::doctype::DocTypeData;
use crate::node::node_data::document::DocumentData;
use crate::node::node_data::element::ElementData;
use crate::node::node_data::text::TextData;
use crate::node::node_data::NodeData;
use crate::node::node_impl;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::node::Node as NodeTrait;

/// The Node struct represents a single node in the DOM tree. It contains a reference to the parent
/// node, a list of children, and the actual data of the node. A node itself does not contain any
/// knowledge of its actual document or storage method, it is just a simple data structure.
#[derive(Debug)]
pub struct Node {
    /// The unique identifier of this node. This is used to reference the node in the document. It
    /// can be `None` when a node is not yet registered in a document.
    id: Option<NodeId>,

    /// List of children (in order) of this node.
    children: Vec<NodeId>,
    /// Optional parent node of this node. This is `None` for the root node.
    parent: Option<NodeId>,

    /// True when the node is registered into a document.
    is_registered: bool,

    /// Actual node data, based on the type of the node
    data: NodeData,
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            children: self.children.clone(),
            parent: self.parent,
            is_registered: self.is_registered,
            data: self.data.clone(),
        }
    }
}

impl NodeTrait for Node
where
    NodeData: From<node_impl::ElementData>
        + From<node_impl::CommentData>
        + From<node_impl::TextData>
        + From<node_impl::DocTypeData>
        + From<node_impl::DocumentData>,
{
    type NodeData = NodeData;
    type ElementData = ElementData;
    type TextData = TextData;
    type CommentData = CommentData;
    type DocTypeData = DocTypeData;
    type DocumentData = DocumentData;

    fn new(data: Self::NodeData) -> Self {
        Self {
            id: None,
            children: Vec::new(),
            parent: None,
            is_registered: false,
            data: data.into(),
        }
    }

    fn id(&self) -> Option<NodeId> {
        self.id
    }

    fn is_registered(&self) -> bool {
        self.is_registered
    }

    fn register(&mut self, id: NodeId) {
        self.is_registered = true;
        self.id = Some(id);
    }

    fn children(&self) -> &Vec<NodeId> {
        self.children.as_ref()
    }

    fn is_renderable(&self) -> bool {
        match self.data {
            NodeData::Element(_) => true,
            NodeData::Text(_) => true,
            NodeData::Comment(_) => false,
            NodeData::DocType(_) => false,
            NodeData::Document(_) => false,
        }
    }

    fn add_child_at_position(&mut self, id: NodeId, position: Option<usize>) {
        match position {
            Some(position) => {
                if position >= self.children.len() {
                    self.children.push(id);
                } else {
                    self.children.insert(position, id);
                }
            }
            None => {
                self.children.push(id);
            }
        }
    }

    fn get_element_data_mut(&mut self) -> Option<&mut Self::ElementData> {
        match self.data {
            NodeData::Element(ref mut data) => Some(data),
            _ => None,
        }
    }

    /// Retrieves the element data of the node in case the node is an element node. If not, it will
    /// return `None`.
    fn get_element_data(&self) -> Option<&Self::ElementData> {
        match self.data {
            NodeData::Element(ref data) => Some(data),
            _ => None,
        }
    }

    fn get_text_data(&self) -> Option<&Self::TextData> {
        match self.data {
            NodeData::Text(ref data) => Some(data),
            _ => None,
        }
    }

    fn get_comment_data(&self) -> Option<&Self::CommentData> {
        match self.data {
            NodeData::Comment(ref data) => Some(data),
            _ => None,
        }
    }

    fn get_doctype_data(&self) -> Option<&Self::DocTypeData> {
        match self.data {
            NodeData::DocType(ref data) => Some(data),
            _ => None,
        }
    }

    fn get_document_data(&self) -> Option<&Self::DocumentData> {
        match self.data {
            NodeData::Document(ref data) => Some(data),
            _ => None,
        }
    }
}
