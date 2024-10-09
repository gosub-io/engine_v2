use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::node_id::NodeId;
use crate::traits::css_system::{CssDeclaration, CssRule, CssValue, HasCssSystem};
use crate::traits::document::HasDocument;
use crate::traits::node::{CommentData, DocTypeData, DocumentData, ElementData, Node, NodeData, TextData};

pub struct MockDocument {
    content: String,
}

pub struct Mock;

pub struct MockNode;

pub struct MockNodeData;

impl NodeData for MockNodeData {
    // type Node = MockNode;
    // type Element = MockElementData;
    // type Text = MockTextData;
    // type Comment = MockCommentData;
    // type DocType = MockDocTypeData;
    // type Document = MockDocumentData;
}

impl From<MockDocumentData> for MockNodeData {
    fn from(data: MockDocumentData) -> Self {
        todo!()
    }
}
impl From<MockElementData> for MockNodeData {
    fn from(data: MockElementData) -> Self {
        todo!()
    }
}
impl From<MockTextData> for MockNodeData {
    fn from(data: MockTextData) -> Self {
        todo!()
    }
}
impl From<MockCommentData> for MockNodeData {
    fn from(data: MockCommentData) -> Self {
        todo!()
    }
}
impl From<MockDocTypeData> for MockNodeData {
    fn from(data: MockDocTypeData) -> Self {
        todo!()
    }
}

pub struct MockElementData;

impl ElementData for MockElementData {
    fn new(name: &str, namespace: &str) -> Self {
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

    fn add_attribute(&mut self, name: &str, value: &str) {
        todo!()
    }

    fn remove_attribute(&mut self, name: &str) {
        todo!()
    }

    fn classes(&self) -> &HashMap<String, bool> {
        todo!()
    }

    fn active_classes(&self) -> Vec<String> {
        todo!()
    }

    fn add_class(&mut self, name: &str, active: bool) {
        todo!()
    }

    fn remove_class(&mut self, name: &str) {
        todo!()
    }

    fn set_class_state(&mut self, name: &str, active: bool) {
        todo!()
    }
}

pub struct MockTextData;

impl TextData for MockTextData {
    fn new(content: &str) -> Self {
        todo!()
    }

    fn content(&self) -> &str {
        todo!()
    }
}

pub struct MockCommentData;

impl CommentData for MockCommentData {
    fn new(content: &str) -> Self {
        todo!()
    }

    fn content(&self) -> &str {
        todo!()
    }
}

pub struct MockDocTypeData;

impl DocTypeData for MockDocTypeData {
    fn new(name: &str, public_id: &str, system_id: &str) -> Self {
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

    fn new(data: Self::NodeData) -> Self {
        todo!()
    }

    fn id(&self) -> Option<NodeId> {
        todo!()
    }

    fn is_registered(&self) -> bool {
        todo!()
    }

    fn register(&mut self, id: NodeId) {
        todo!()
    }

    fn children(&self) -> &Vec<NodeId> {
        todo!()
    }

    fn is_renderable(&self) -> bool {
        todo!()
    }

    fn add_child_at_position(&mut self, id: NodeId, position: Option<usize>) {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl CssRule for MockCssRule {
    fn new() -> Self {
        todo!()
    }

    fn add_selector(&mut self, selector: &str) {
        todo!()
    }

    fn add_declaration(&mut self, declaration: Self::CssDeclaration) {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl CssDeclaration for MockCssDeclaration {
    fn new(property: &str, value: Self::CssValue, important: bool) -> Self {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl CssValue for MockCssValue {
    fn unit(value: f32, unit: &str) -> Self {
        todo!()
    }

    fn keyword(value: &str) -> Self {
        todo!()
    }

    fn colorvalue(value: &str) -> Self {
        todo!()
    }

    fn list(args: Vec<Self>) -> Self {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl crate::traits::css_system::CssStylesheet for MockCssStylesheet {
    fn new() -> Self {
        MockCssStylesheet
    }

    fn add_rule(&mut self, rule: MockCssRule) {
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

impl HasDocument for Mock {
    type Document = MockDocument;
    type Node = MockNode;
}
