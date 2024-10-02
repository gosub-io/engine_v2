use gosub_shared::node_id::NodeId;
use gosub_shared::traits::node::{Node as NodeTrait};
use crate::node::node_data::NodeData;
use crate::node::node_data::comment::CommentData;
use crate::node::node_data::doctype::DocTypeData;
use crate::node::node_data::document::DocumentData;
use crate::node::node_data::element::ElementData;
use crate::node::node_data::text::TextData;

#[derive(Debug, Clone)]
pub struct Node {
    id: Option<NodeId>,

    children: Vec<NodeId>,
    parent: Option<NodeId>,

    is_registered: bool,

    data: NodeData,
}

impl NodeTrait for Node
where
    NodeData: From<ElementData> + From<CommentData> + From<TextData> + From<DocTypeData>  + From<DocumentData>
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

    fn add_child_at_position(&mut self, id: NodeId, position: Option<usize>) {
        println!("I'm node: {:?}, adding child {:?} at position: {:?}", self.id, id, position);
        match position {
            Some(position) => {
                if position >= self.children.len() {
                    self.children.push(id);
                } else {
                    self.children.insert(position, id);
                }
            },
            None => {
                self.children.push(id);
            },
        }
    }

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