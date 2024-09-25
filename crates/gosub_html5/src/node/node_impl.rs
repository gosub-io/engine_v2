use std::collections::HashMap;
use gosub_shared::node_id::NodeId;

#[derive(Debug, Clone)]
pub struct ElementAttribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Element,
    Text,
    Comment,
    DocType,
}

#[derive(Debug, Clone)]
pub struct ElementData {
    name: String,
    namespace: String,
    attributes: Vec<ElementAttribute>,
    classes: HashMap<String, bool>,     // classname -> is active or not
}

impl ElementData {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn namespace(&self) -> &str {
        self.namespace.as_str()
    }

    pub fn attributes(&self) -> &Vec<ElementAttribute> {
        &self.attributes
    }

    pub fn add_attribute(&mut self, name: &str, value: &str) {
        self.attributes.push(ElementAttribute { name: name.into(), value: value.into() });

        // @todo: do things with "class" and "id"
        if name == "class" {
            for class in value.split_whitespace() {
                self.classes.insert(class.to_string(), true);
            }
        }
    }

    pub fn remove_attribute(&mut self, name: String) {
        self.attributes.retain(|attr| attr.name != name);
    }

    pub fn classes(&self) -> &HashMap<String, bool> {
        &self.classes
    }

    pub fn active_classes(&self) -> Vec<String> {
        self.classes.iter().filter(|(_, &active)| active).map(|(name,_)| name.to_string()).collect()
    }

    pub fn add_class(&mut self, name: String, active: bool) {
        self.classes.insert(name, active);
    }

    pub fn remove_class(&mut self, name: String) {
        self.classes.remove(&name);
    }

    pub fn set_class_state(&mut self, name: String, active: bool) {
        self.classes.insert(name, active);
    }

    pub fn new(name: String, namespace: String) -> Self {
        Self {
            name,
            namespace,
            attributes: Vec::new(),
            classes: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextData {
    pub content: String,
}

impl TextData {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

#[derive(Debug, Clone)]
pub struct CommentData {
    pub content: String,
}

impl CommentData {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

#[derive(Debug, Clone)]
pub struct DocTypeData {
    pub name: String,
    pub public_id: String,
    pub system_id: String,
}

impl DocTypeData {
    pub fn new(name: String, public_id: String, system_id: String) -> Self {
        Self {
            name,
            public_id,
            system_id,
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeData {
    Element(ElementData),
    Text(TextData),
    Comment(CommentData),
    DocType(DocTypeData),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: Option<NodeId>,

    pub children: Vec<NodeId>,
    pub parent: Option<NodeId>,

    pub is_registered: bool,

    pub data: NodeData,
}

impl Node {
    fn new(data: NodeData) -> Self {
        Self {
            id: None,
            children: Vec::new(),
            parent: None,
            is_registered: false,
            data,
        }
    }

    pub fn is_registered(&self) -> bool {
        self.is_registered
    }

    pub fn register(&mut self, id: NodeId) {
        self.is_registered = true;
        self.id = Some(id);
    }

    pub fn new_element_node(name: String, namespace: String) -> Self {
        Self::new(NodeData::Element(ElementData::new(name, namespace)))
    }

    pub fn new_text_node(content: String) -> Self {
        Self::new(NodeData::Text(TextData::new(content)))
    }

    pub fn new_comment_node(content: String) -> Self {
        Self::new(NodeData::Comment(CommentData::new(content)))
    }

    pub fn new_doctype_node(name: String, public_id: String, system_id: String) -> Self {
        Self::new(NodeData::DocType(DocTypeData::new(name, public_id, system_id)))
    }

    pub fn nodetype(&self) -> NodeType {
        match self.data {
            NodeData::Element(_) => NodeType::Element,
            NodeData::Text(_) => NodeType::Text,
            NodeData::Comment(_) => NodeType::Comment,
            NodeData::DocType(_) => NodeType::DocType,
        }
    }

    pub fn get_element_data(&self) -> Option<&ElementData> {
        match self.data {
            NodeData::Element(ref data) => Some(data),
            _ => None,
        }
    }

    pub fn get_text_data(&self) -> Option<&TextData> {
        match self.data {
            NodeData::Text(ref data) => Some(data),
            _ => None,
        }
    }

    pub fn get_comment_data(&self) -> Option<&CommentData> {
        match self.data {
            NodeData::Comment(ref data) => Some(data),
            _ => None,
        }
    }

    pub fn get_doctype_data(&self) -> Option<&DocTypeData> {
        match self.data {
            NodeData::DocType(ref data) => Some(data),
            _ => None,
        }
    }
}