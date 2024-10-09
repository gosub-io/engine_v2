use gosub_shared::document::DocumentHandle;
use gosub_shared::location::Location;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::node::{Node, NodeBuilder as _};
use crate::node::builder::NodeBuilder;

pub enum DocumentTask {
    CreateElement {
        name: String,
        namespace: String,
        parent_id: NodeId,
        position: Option<usize>,
        location: Location
    },
    CreateText {
        content: String,
        parent_id: NodeId,
        location: Location
    },
    CreateComment {
        content: String,
        parent_id: NodeId,
        location: Location
    },
    InsertAttribute {
        key: String,
        value: String,
        node_id: NodeId,
        location: Location
    }
}

pub struct DocumentTaskQueue<C: HasDocument> {    
    doc_handle: DocumentHandle<C>,
    pending_tasks: Vec<DocumentTask>,
}

impl <C: HasDocument> DocumentTaskQueue<C> {
    pub fn new(doc_handle: DocumentHandle<C>) -> Self {
        Self {
            doc_handle,
            pending_tasks: Vec::new(),
        }
    }

    pub fn has_pending_tasks(&self) -> bool {
        !self.pending_tasks.is_empty()
    }

    pub fn add_task(&mut self, task: DocumentTask) {
        self.pending_tasks.push(task);
    }

    pub fn flush(&mut self) -> Vec<String> {
        let mut errors = Vec::new();

        for current_task in &self.pending_tasks {
            match current_task {
                DocumentTask::CreateElement { name, namespace, parent_id, position, location: _location } => {
                    let mut binding = self.doc_handle.get_mut();

                    let new_node = NodeBuilder::new_element_node(name, namespace);
                    binding.register_node_at(new_node, *parent_id, *position);
                }
                DocumentTask::CreateText { content, parent_id, location: _location } => {
                    let mut binding = self.doc_handle.get_mut();

                    let new_node = NodeBuilder::new_text_node(content);
                    binding.register_node_at(new_node, *parent_id, None);
                }
                DocumentTask::CreateComment { content, parent_id, location: _location } => {
                    let mut binding = self.doc_handle.get_mut();

                    let new_node = NodeBuilder::new_comment_node(content);
                    binding.register_node_at(new_node, *parent_id, None);
                }
                DocumentTask::InsertAttribute { key, value, node_id, location: _location } => {
                    let mut binding = self.doc_handle.get_mut();

                    if let Some(node) = binding.detach_node(*node_id) {
                        if !node.is_element() {
                            binding.update_node(node_id, node);

                            errors.push(format!("Node with id {} is not an element node", node_id));
                            continue;
                        }

                        node.insert_attribute(key, value);
                        binding.update_node(node_id, node);
                    } else {
                        errors.push(format!("Node with id {} not found", node_id));
                    }
                }
            }
        }

        self.pending_tasks.clear();

        errors
    }
}