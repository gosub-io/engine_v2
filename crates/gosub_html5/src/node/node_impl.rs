use gosub_shared::node_id::NodeId;
use std::collections::HashMap;
use std::fmt::Display;
use gosub_shared::traits::node::{ElementData as ElementDataTrait, TextData as TextDataTrait, CommentData as CommentDataTrait, DocTypeData as DocTypeDataTrait, HasNode};

#[derive(Debug, Clone)]
pub struct ElementAttribute {
    name: String,
    value: String,
}

impl gosub_shared::traits::node::ElementAttribute for ElementAttribute {
    fn new(name: &str, value: &str) -> Self {
        Self { name: name.into(), value: value.into() }
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn value(&self) -> &str {
        self.value.as_str()
    }
}

impl Display for ElementAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}=\"{}\"", self.name, self.value)
    }
}

#[derive(Debug, Clone)]
pub struct ElementData {
    name: String,
    namespace: String,
    attributes: Vec<ElementAttribute>,
    classes: HashMap<String, bool>,     // classname -> is active or not
}

impl gosub_shared::traits::node::NodeData for ElementData {}

impl gosub_shared::traits::node::ElementData for ElementData {
    fn new(name: &str, namespace: &str) -> Self {
        Self {
            name: name.into(),
            namespace: namespace.into(),
            attributes: Vec::new(),
            classes: HashMap::new(),
        }
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn namespace(&self) -> &str {
        self.namespace.as_str()
    }

    fn attributes(&self) -> &Vec<impl gosub_shared::traits::node::ElementAttribute> {
        &self.attributes
    }

    fn add_attribute(&mut self, name: &str, value: &str) {
        self.attributes.push(ElementAttribute { name: name.into(), value: value.into() });

        // @todo: do things with "class" and "id"
        if name == "class" {
            for class in value.split_whitespace() {
                self.classes.insert(class.to_string(), true);
            }
        }
    }

    fn remove_attribute(&mut self, name: &str) {
        self.attributes.retain(|attr| attr.name != name);
    }

    fn classes(&self) -> &HashMap<String, bool> {
        &self.classes
    }

    fn active_classes(&self) -> Vec<String> {
        self.classes.iter().filter(|(_, &active)| active).map(|(name, _)| name.to_string()).collect()
    }

    fn add_class(&mut self, name: &str, active: bool) {
        self.classes.insert(name.into(), active);
    }

    fn remove_class(&mut self, name: &str) {
        self.classes.remove(name);
    }

    fn set_class_state(&mut self, name: &str, active: bool) {
        self.classes.insert(name.into(), active);
    }
}

#[derive(Debug, Clone)]
pub struct TextData {
    content: String,
}

impl gosub_shared::traits::node::NodeData for TextData {}

impl gosub_shared::traits::node::TextData for TextData {
    fn new(content: &str) -> Self {
        Self { content: content.into() }
    }

    fn content(&self) -> &str {
        self.content.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct CommentData {
    content: String,
}

impl gosub_shared::traits::node::NodeData for CommentData {}

impl gosub_shared::traits::node::CommentData for CommentData {
    fn new(content: &str) -> Self {
        Self { content: content.into() }
    }

    fn content(&self) -> &str {
        self.content.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct DocTypeData {
    name: String,
    public_id: String,
    system_id: String,
}

impl gosub_shared::traits::node::NodeData for DocTypeData {}

impl gosub_shared::traits::node::DocTypeData for DocTypeData {
    fn new(name: &str, public_id: &str, system_id: &str) -> Self {
        Self {
            name: name.into(),
            public_id: public_id.into(),
            system_id: system_id.into(),
        }
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn public_id(&self) -> &str {
        self.public_id.as_str()
    }

    fn system_id(&self) -> &str {
        self.system_id.as_str()
    }
}

#[derive(Debug, Clone)]
pub enum NodeData {
    Element(ElementData),
    Text(TextData),
    Comment(CommentData),
    DocType(DocTypeData),
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

#[derive(Debug, Clone)]
pub struct Node {
    id: Option<NodeId>,

    children: Vec<NodeId>,
    parent: Option<NodeId>,

    is_registered: bool,

    data: NodeData,
}

impl HasNode for Node {
    type Node = Node;
}

impl gosub_shared::traits::node::Node for Node {
    type NodeData = NodeData;

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

    fn get_element_data(&self) -> Option<&impl ElementDataTrait> {
        match self.data {
            NodeData::Element(ref data) => Some(data),
            _ => None,
        }
    }

    fn get_text_data(&self) -> Option<&impl TextDataTrait> {
        match self.data {
            NodeData::Text(ref data) => Some(data),
            _ => None,
        }
    }

    fn get_comment_data(&self) -> Option<&impl CommentDataTrait> {
        match self.data {
            NodeData::Comment(ref data) => Some(data),
            _ => None,
        }
    }

    fn get_doctype_data(&self) -> Option<&impl DocTypeDataTrait> {
        match self.data {
            NodeData::DocType(ref data) => Some(data),
            _ => None,
        }
    }
}