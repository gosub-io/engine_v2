use std::collections::HashMap;
use std::fmt::Display;
use crate::node_id::NodeId;

// As long as the enum NodeData implements this trait, we don't have to specify in this shared crate
pub trait NodeData: Sized {}

pub trait ElementAttribute: Display {
    fn new(name: &str, value: &str) -> Self;
    fn name(&self) -> &str;
    fn value(&self) -> &str;
}

pub trait ElementData {
    fn new(name: &str, namespace: &str) -> Self;
    fn name(&self) -> &str;
    fn namespace(&self) -> &str;
    fn attributes(&self) -> &Vec<impl ElementAttribute>;
    fn add_attribute(&mut self, name: &str, value: &str);
    fn remove_attribute(&mut self, name: &str);
    fn classes(&self) -> &HashMap<String, bool>;
    fn active_classes(&self) -> Vec<String>;
    fn add_class(&mut self, name: &str, active: bool);
    fn remove_class(&mut self, name: &str);
    fn set_class_state(&mut self, name: &str, active: bool);
}

pub trait TextData {
    fn new(content: &str) -> Self;
    fn content(&self) -> &str;
}

pub trait CommentData {
    fn new(content: &str) -> Self;
    fn content(&self) -> &str;
}

pub trait DocTypeData {
    fn new(name: &str, public_id: &str, system_id: &str) -> Self;
    fn name(&self) -> &str;
    fn public_id(&self) -> &str;
    fn system_id(&self) -> &str;
}

pub trait DocumentData {
    fn new() -> Self;
}

pub trait Node: Sized
where
    Self::NodeData: From<Self::ElementData> + From<Self::CommentData> + From<Self::TextData> + From<Self::DocTypeData> + From<Self::DocumentData>
{
    type NodeData: NodeData;
    type ElementData: ElementData;
    type TextData: TextData;
    type CommentData: CommentData;
    type DocTypeData: DocTypeData;
    type DocumentData: DocumentData;

    fn new(data: Self::NodeData) -> Self;
    fn id(&self) -> Option<NodeId>;

    fn is_registered(&self) -> bool;
    fn register(&mut self, id: NodeId);

    fn children(&self) -> &Vec<NodeId>;
    fn add_child_at_position(&mut self, id: NodeId, position: Option<usize>);

    fn get_element_data_mut(&mut self) -> Option<&mut Self::ElementData>;
    fn get_element_data(&self) -> Option<&Self::ElementData>;
    fn get_text_data(&self) -> Option<&Self::TextData>;
    fn get_comment_data(&self) -> Option<&Self::CommentData>;
    fn get_doctype_data(&self) -> Option<&Self::DocTypeData>;
    fn get_document_data(&self) -> Option<&Self::DocumentData>;
}

pub trait NodeBuilder<N: Node>: Sized {
    fn new_element_node(name: &str, namespace: &str) -> N;
    fn new_text_node(content: &str) -> N;
    fn new_comment_node(content: &str) -> N;
    fn new_doctype_node(name: &str, public_id: &str, system_id: &str) -> N;
    fn new_document_node() -> N;
}

// pub trait HasNode: Sized {
//     type Node: Node;
//     // type NodeBuilder: NodeBuilder<Self::Node>;
// }