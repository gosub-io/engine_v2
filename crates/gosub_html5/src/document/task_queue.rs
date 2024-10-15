use gosub_shared::document::DocumentHandle;
use gosub_shared::location::Location;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::node::{ElementData, Node, NodeBuilder as _};
use crate::node::builder::NodeBuilder;

/// The Document Task Queue is used to queue up tasks that need to be executed on the document. This
/// can be used to defer tasks to a later stage.

/// Actual task that needs to be completed on flush
#[derive(Clone, Debug)]
pub struct Task {
    /// Node ID that this task is related to. It's either the node-id reserved when creating a node
    /// or an existing node-id when inserting an attribute.
    pub node_id: NodeId,
    pub task: DocumentTask,
}

impl Task {
    pub fn new(node_id: NodeId, task: DocumentTask) -> Self {
        Self {
            node_id,
            task,
        }
    }
}

#[derive(Clone, Debug)]
pub enum DocumentTask {
    /// Create a new element node
    CreateElement {
        name: String,
        namespace: String,
        location: Location,
        parent_id: NodeId,
        position: Option<usize>,
    },
    /// Create a new text node
    CreateText {
        content: String,
        location: Location,
        parent_id: NodeId,
        position: Option<usize>,
    },
    /// Create a new comment node
    #[allow(dead_code)]
    CreateComment {
        content: String,
        location: Location,
        parent_id: NodeId,
        position: Option<usize>,
    },
    /// Insert an attribute into an element node
    InsertAttribute {
        node_id: NodeId,
        key: String,
        value: String,
        location: Location,
    }
}

pub struct DocumentTaskQueue<C: HasDocument> {
    /// Handle to the document
    doc_handle: DocumentHandle<C>,
    /// Pending tasks that are needed to be flushed
    pending_tasks: Vec<Task>,
}

impl <C: HasDocument> DocumentTaskQueue<C> {
    pub fn new(doc_handle: DocumentHandle<C>) -> Self {
        Self {
            doc_handle,
            pending_tasks: Vec::new(),
        }
    }

    /// Returns true if there are pending tasks in the queue
    #[allow(dead_code)]
    pub fn has_pending_tasks(&self) -> bool {
        !self.pending_tasks.is_empty()
    }

    /// Add a new task to the queue on the given destination. Returns a reserved node_id (that isn't
    /// used yet, but will be once flushed) of the new node.
    pub fn add_task(&mut self, task: DocumentTask) -> NodeId {
        let node_id = match task {
            DocumentTask::InsertAttribute { node_id, .. } => {
                // For insertions of attributes, we don't need to reserve a new node_id
                node_id
            }
            _ => {
                self.doc_handle.get_mut().get_next_node_id()
            }
        };

        self.pending_tasks.push(Task::new(node_id, task));

        node_id
    }

    /// Executes all pending tasks into the document
    pub fn flush(&mut self) -> Vec<String> {
        let mut errors = Vec::new();

        for current_task in self.pending_tasks.clone() {
            match current_task.task {
                DocumentTask::CreateElement { name, namespace, location: _location, parent_id, position } => {
                    let new_node = NodeBuilder::new_element_node(name.as_str(), namespace.as_str());
                    self.insert_node(current_task.node_id, new_node, parent_id, position);
                }
                DocumentTask::CreateText { content, location: _location, parent_id, position } => {
                    let new_node = NodeBuilder::new_text_node(content.as_str());
                    self.insert_node(current_task.node_id, new_node, parent_id, position);
                }
                DocumentTask::CreateComment { content, location: _location, parent_id, position } => {
                    let new_node = NodeBuilder::new_comment_node(content.as_str());
                    self.insert_node(current_task.node_id, new_node, parent_id, position);
                }
                DocumentTask::InsertAttribute { key, value, location: _location, node_id } => {
                    let mut binding = self.doc_handle.get_mut();
                    if let Some(mut node) = binding.detach_node(node_id) {
                        if let Some(element) = node.get_element_data_mut() {
                            element.add_attribute(&key, &value);
                        } else {
                            errors.push(format!("Node with id {} is not an element node", node_id));
                        }
                        binding.update_node(node_id, node);
                    } else {
                        errors.push(format!("Node with id {} not found", node_id));
                    }
                }
            };
        }

        self.pending_tasks.clear();

        errors
    }

    /// Insert a node into the document
    fn insert_node(&mut self, node_id: NodeId, mut node: C::Node, parent_id: NodeId, position: Option<usize>) {
        let mut binding = self.doc_handle.get_mut();
        node.register(node_id);

        binding.register_node_at(node, parent_id, position);
    }
}