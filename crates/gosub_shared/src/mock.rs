use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use anyhow::Error;
use crate::document::DocumentHandle;
use crate::node_id::NodeId;
use crate::traits::css_system::{CssDeclaration, CssRule, CssValue, HasCssSystem};
use crate::traits::document::{Document, HasDocument};
use crate::traits::document::query::{Condition, Query, SearchType};
use crate::traits::node::{CommentData, DocTypeData, DocumentData, ElementData, Node, NodeData, TextData};

pub struct MockDocument<C: HasDocument> {
    pub(crate) content: String,
    pub(crate) handle: Option<DocumentHandle<C>>,
}


impl<C: HasDocument<Document = Self>> Debug for MockDocument<C>
where
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MockDocument")
            .field("content", &self.content)
            .field("handle", &self.handle.is_some())
            .finish()
    }
}

pub struct Mock;

pub struct MockNode;

pub struct MockNodeData;

pub struct MockQuery;

#[derive(PartialEq)]
pub enum MockSearchType {
}

impl SearchType for MockSearchType {
    fn uninitialized() -> Self {
        todo!()
    }

    fn find_first() -> Self {
        todo!()
    }

    fn find_all() -> Self {
        todo!()
    }
}

pub enum MockCondition {
}

impl Condition for MockCondition {
    fn equals_tag(_tag_name: &str) -> Self {
        todo!()
    }

    fn equals_id(_id: &str) -> Self {
        todo!()
    }

    fn contains_class(_class: &str) -> Self {
        todo!()
    }

    fn contains_attribute(_attribute: &str) -> Self {
        todo!()
    }

    fn contains_child_tag(_child_tag: &str) -> Self {
        todo!()
    }

    fn has_parent_tag(_parent_tag: &str) -> Self {
        todo!()
    }
}

impl Query for MockQuery {
    type SearchType = MockSearchType;
    type Condition = MockCondition;

    fn new(_search_type: Self::SearchType, _conditions: Vec<Self::Condition>) -> Self {
        todo!()
    }

    fn search_type(&self) -> Self::SearchType {
        todo!()
    }

    fn conditions(&self) -> Vec<Self::Condition> {
        todo!()
    }
}

impl NodeData for MockNodeData {
    // type Node = MockNode;
    // type Element = MockElementData;
    // type Text = MockTextData;
    // type Comment = MockCommentData;
    // type DocType = MockDocTypeData;
    // type Document = MockDocumentData;
}

impl From<MockDocumentData> for MockNodeData {
    fn from(_data: MockDocumentData) -> Self {
        todo!()
    }
}
impl From<MockElementData> for MockNodeData {
    fn from(_data: MockElementData) -> Self {
        todo!()
    }
}
impl From<MockTextData> for MockNodeData {
    fn from(_data: MockTextData) -> Self {
        todo!()
    }
}
impl From<MockCommentData> for MockNodeData {
    fn from(_data: MockCommentData) -> Self {
        todo!()
    }
}
impl From<MockDocTypeData> for MockNodeData {
    fn from(_data: MockDocTypeData) -> Self {
        todo!()
    }
}

pub struct MockElementData;

impl ElementData for MockElementData {
    fn new(_name: &str, _namespace: &str) -> Self {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn namespace(&self) -> &str {
        todo!()
    }

    fn attributes(&self) -> &HashMap<String, String> {
        todo!()
    }

    fn add_attribute(&mut self, _name: &str, _value: &str) {
        todo!()
    }

    fn remove_attribute(&mut self, _name: &str) {
        todo!()
    }

    fn classes(&self) -> &HashMap<String, bool> {
        todo!()
    }

    fn active_classes(&self) -> Vec<String> {
        todo!()
    }

    fn add_class(&mut self, _name: &str, _active: bool) {
        todo!()
    }

    fn remove_class(&mut self, _name: &str) {
        todo!()
    }

    fn set_class_state(&mut self, _name: &str, _active: bool) {
        todo!()
    }
}

pub struct MockTextData;

impl TextData for MockTextData {
    fn new(_content: &str) -> Self {
        todo!()
    }

    fn content(&self) -> &str {
        todo!()
    }
}

pub struct MockCommentData;

impl CommentData for MockCommentData {
    fn new(_content: &str) -> Self {
        todo!()
    }

    fn content(&self) -> &str {
        todo!()
    }
}

pub struct MockDocTypeData;

impl DocTypeData for MockDocTypeData {
    fn new(_name: &str, _public_id: &str, _system_id: &str) -> Self {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn public_id(&self) -> &str {
        todo!()
    }

    fn system_id(&self) -> &str {
        todo!()
    }
}

pub struct MockDocumentData;

impl DocumentData for MockDocumentData {
    fn new() -> Self {
        todo!()
    }
}

impl Node for MockNode {
    type NodeData = MockNodeData;
    type ElementData = MockElementData;
    type TextData = MockTextData;
    type CommentData = MockCommentData;
    type DocTypeData = MockDocTypeData;
    type DocumentData = MockDocumentData;

    fn new(_data: Self::NodeData) -> Self {
        todo!()
    }

    fn id(&self) -> Option<NodeId> {
        todo!()
    }

    fn is_registered(&self) -> bool {
        todo!()
    }

    fn register(&mut self, _node_id: NodeId) {
        todo!()
    }

    fn children(&self) -> &Vec<NodeId> {
        todo!()
    }

    fn is_renderable(&self) -> bool {
        todo!()
    }

    fn add_child_at_position(&mut self, _node_id: NodeId, _position: Option<usize>) {
        todo!()
    }

    fn get_element_data_mut(&mut self) -> Option<&mut Self::ElementData> {
        todo!()
    }

    fn get_element_data(&self) -> Option<&Self::ElementData> {
        todo!()
    }

    fn get_text_data(&self) -> Option<&Self::TextData> {
        todo!()
    }

    fn get_comment_data(&self) -> Option<&Self::CommentData> {
        todo!()
    }

    fn get_doctype_data(&self) -> Option<&Self::DocTypeData> {
        todo!()
    }

    fn get_document_data(&self) -> Option<&Self::DocumentData> {
        todo!()
    }
}

pub struct MockCssStylesheet;

pub struct MockCssRule;

impl HasCssSystem for MockCssRule {
    type CssStylesheet = MockCssStylesheet;
    type CssRule = MockCssRule;
    type CssDeclaration = MockCssDeclaration;
    type CssValue = MockCssValue;
}

impl Debug for MockCssRule {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl CssRule for MockCssRule {
    fn new() -> Self {
        todo!()
    }

    fn add_selector(&mut self, _selector: &str) {
        todo!()
    }

    fn add_declaration(&mut self, _declaration: Self::CssDeclaration) {
        todo!()
    }

    fn selectors(&self) -> &Vec<String> {
        todo!()
    }

    fn declarations(&self) -> &Vec<Self::CssDeclaration> {
        todo!()
    }
}

pub struct MockCssDeclaration;

impl HasCssSystem for MockCssDeclaration {
    type CssStylesheet = MockCssStylesheet;
    type CssRule = MockCssRule;
    type CssDeclaration = MockCssDeclaration;
    type CssValue = MockCssValue;
}

impl Debug for MockCssDeclaration {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl CssDeclaration for MockCssDeclaration {
    fn new(_property: &str, _value: Self::CssValue, _important: bool) -> Self {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn value(&self) -> MockCssValue {
        todo!()
    }

    fn important(&self) -> bool {
        todo!()
    }
}

pub struct MockCssValue;

impl Debug for MockCssValue {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl CssValue for MockCssValue {
    fn unit(_value: f32, _unit: &str) -> Self {
        todo!()
    }

    fn keyword(_value: &str) -> Self {
        todo!()
    }

    fn colorvalue(_value: &str) -> Self {
        todo!()
    }

    fn list(_args: Vec<Self>) -> Self {
        todo!()
    }

    fn is_unit(&self) -> bool {
        todo!()
    }

    fn is_keyword(&self) -> bool {
        todo!()
    }

    fn is_color(&self) -> bool {
        todo!()
    }

    fn is_list(&self) -> bool {
        todo!()
    }
}

impl HasCssSystem for MockCssStylesheet {
    type CssStylesheet = MockCssStylesheet;
    type CssRule = MockCssRule;
    type CssDeclaration = MockCssDeclaration;
    type CssValue = MockCssValue;
}

impl Debug for MockCssStylesheet {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl crate::traits::css_system::CssStylesheet for MockCssStylesheet {
    fn new() -> Self {
        MockCssStylesheet
    }

    fn add_rule(&mut self, _rule: MockCssRule) {
        todo!()
    }

    fn rules(&self) -> &Vec<MockCssRule> {
        todo!()
    }
}

impl HasCssSystem for Mock {
    type CssStylesheet = MockCssStylesheet;
    type CssRule = MockCssRule;
    type CssDeclaration = MockCssDeclaration;
    type CssValue = MockCssValue;
}

// impl HasDocument for Mock {
// }

impl<C: HasDocument> HasCssSystem for MockDocument<C> {
    type CssStylesheet = MockCssStylesheet;
    type CssRule = MockCssRule;
    type CssDeclaration = MockCssDeclaration;
    type CssValue = MockCssValue;
}


impl<C: HasDocument> Document<C> for MockDocument<C> {
    type Node = MockNode;
    type Query = MockQuery;
    type Document = Self;

    fn new(_url: &str) -> Self {
        todo!()
    }

    fn register_node_at(&mut self, _node: Self::Node, _parent_id: NodeId, _position: Option<usize>) -> NodeId {
        todo!()
    }

    fn get_next_node_id(&mut self) -> NodeId {
        todo!()
    }

    fn get_handle(&self) -> DocumentHandle<C> {
        self.handle.clone().unwrap()
    }

    fn set_handle(&mut self, handle: DocumentHandle<C>) {
        self.handle = Some(handle);
    }

    fn get_root_node(&self) -> Option<&Self::Node> {
        todo!()
    }

    fn get_node(&self, _id: NodeId) -> Option<&Self::Node> {
        todo!()
    }

    fn stylesheets(&self) -> &Vec<C::CssStylesheet> {
        todo!()
    }

    fn add_stylesheet(&mut self, _stylesheet: C::CssStylesheet) {
        todo!()
    }

    fn detach_node(&mut self, _id: NodeId) -> Option<Self::Node> {
        todo!()
    }

    fn update_node(&mut self, _id: NodeId, _node: Self::Node) {
        todo!()
    }

    fn get_url(&self) -> &str {
        todo!()
    }

    fn get_node_mut(&mut self, _id: NodeId) -> Option<&mut Self::Node> {
        todo!()
    }

    fn get_node_clone(&self, _id: NodeId) -> Option<Self::Node> {
        todo!()
    }

    fn get_node_by_element_id(&self, _name: &str) -> Option<NodeId> {
        todo!()
    }

    fn query(&self, _query: &Self::Query) -> Result<Vec<NodeId>, Error> {
        todo!()
    }
}


pub struct MockConfig;

impl HasCssSystem for MockConfig {
    type CssStylesheet = MockCssStylesheet;
    type CssRule = MockCssRule;
    type CssDeclaration = MockCssDeclaration;
    type CssValue = MockCssValue;
}

impl HasDocument for MockConfig {
    type Document = MockDocument<Self>;
    type Node = MockNode;
}
