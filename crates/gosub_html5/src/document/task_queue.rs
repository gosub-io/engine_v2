use std::collections::HashMap;
use anyhow::Error;
use gosub_shared::document::DocumentHandle;
use gosub_shared::location::Location;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::node::{ElementData, Node, NodeBuilder as _};
use crate::node::builder::NodeBuilder;


/// Identifier for each task found in the queue. Can be recycled after a flush
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TaskId(usize);

/// Destination of the task. It could be a node, the root, or another task that is currently in the task queue
#[derive(Clone, Debug)]
pub enum TaskDestination {
    /// Destination is an existing node
    #[allow(dead_code)]
    Node(NodeId, Option<usize>),
    /// Destination is a task that is currently in the task queue (does not have a node-id yet)
    Task(TaskId, Option<usize>),
    /// Destination is the root node
    DocumentRoot(Option<usize>),
}

/// Actual task that needs to be completed on flush
#[derive(Clone, Debug)]
pub struct Task {
    pub task_id: TaskId,
    pub task: DocumentTask,
    pub destination: TaskDestination,
}

impl Task {
    pub fn new(task_id: TaskId, task: DocumentTask, destination: TaskDestination) -> Self {
        Self {
            task_id,
            task,
            destination,
        }
    }
}

#[derive(Clone, Debug)]
pub enum DocumentTask {
    /// Create a new element node
    CreateElement {
        name: String,
        namespace: String,
        location: Location
    },
    /// Create a new text node
    CreateText {
        content: String,
        location: Location
    },
    /// Create a new comment node
    #[allow(dead_code)]
    CreateComment {
        content: String,
        location: Location
    },
    /// Insert an attribute into an element node
    InsertAttribute {
        key: String,
        value: String,
        location: Location
    }
}

pub struct DocumentTaskQueue<C: HasDocument> {
    /// Handle to the document
    doc_handle: DocumentHandle<C>,
    /// Pending tasks that are needed to be flushed
    pending_tasks: Vec<Task>,
    /// Next task id
    next_task_id: usize,
    /// Mapping of task id to node id in case we reference a task id in a task's destination
    task_nodes: HashMap<TaskId, NodeId>,
}

impl <C: HasDocument> DocumentTaskQueue<C> {
    pub fn new(doc_handle: DocumentHandle<C>) -> Self {
        Self {
            doc_handle,
            pending_tasks: Vec::new(),
            next_task_id: 0,
            task_nodes: HashMap::new(),
        }
    }

    /// Returns true if there are pending tasks in the queue
    #[allow(dead_code)]
    pub fn has_pending_tasks(&self) -> bool {
        !self.pending_tasks.is_empty()
    }

    /// Add a new task to the queue on the given destination
    pub fn add_task(&mut self, task: DocumentTask, destination: TaskDestination) -> TaskId {
        self.next_task_id += 1;
        let task_id = TaskId(self.next_task_id);

        self.pending_tasks.push(Task::new(task_id, task, destination));

        task_id
    }

    /// Executes all pending tasks into the document
    pub fn flush(&mut self) -> Vec<String> {
        let mut errors = Vec::new();

        for current_task in self.pending_tasks.clone() {
            match current_task.task {
                DocumentTask::CreateElement { name, namespace, location: _location } => {
                    let new_node = NodeBuilder::new_element_node(name.as_str(), namespace.as_str());
                    match self.insert_node(new_node, current_task.destination, current_task.task_id) {
                        Ok(_) => {},
                        Err(e) => errors.push(e.to_string()),
                    }
                }
                DocumentTask::CreateText { content, location: _location } => {
                    let new_node = NodeBuilder::new_text_node(content.as_str());
                    match self.insert_node(new_node, current_task.destination, current_task.task_id) {
                        Ok(_) => {},
                        Err(e) => errors.push(e.to_string()),
                    }
                }
                DocumentTask::CreateComment { content, location: _location } => {
                    let new_node = NodeBuilder::new_comment_node(content.as_str());
                    match self.insert_node(new_node, current_task.destination, current_task.task_id) {
                        Ok(_) => {},
                        Err(e) => errors.push(e.to_string()),
                    }
                }
                DocumentTask::InsertAttribute { key, value, location: _location } => {
                    // Find node_id based on destination
                    let node_id = match current_task.destination {
                        TaskDestination::Node(node_id, _) => node_id,
                        TaskDestination::Task(task_id, _) => {
                            if !self.task_nodes.contains_key(&task_id) {
                                errors.push(format!("Task id {:?} not found", task_id));
                                continue;
                            }
                            self.task_nodes[&task_id]
                        },
                        TaskDestination::DocumentRoot(_) => NodeId::root(),
                    };

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
        self.task_nodes.clear();

        errors
    }

    /// Insert a node into the document
    fn insert_node(&mut self, node: C::Node, destination: TaskDestination, task_id: TaskId) -> Result<NodeId, Error> {
        let mut binding = self.doc_handle.get_mut();

        match destination {
            TaskDestination::Node(parent_id, position) => {
                let node_id = binding.register_node_at(node, parent_id, position);

                self.task_nodes.insert(task_id, node_id);

                Ok(node_id)
            }
            TaskDestination::Task(dest_task_id, position) => {
                if ! self.task_nodes.contains_key(&dest_task_id) {
                    return Err(anyhow::anyhow!("Task id {:?} not found", dest_task_id));
                }
                let parent_node_id = self.task_nodes[&dest_task_id];
                let node_id = binding.register_node_at(node, parent_node_id, position);

                self.task_nodes.insert(task_id, node_id);

                Ok(node_id)
            }
            TaskDestination::DocumentRoot(position) => {
                let node_id = NodeId::root();
                binding.register_node_at(node, node_id, position);
                self.task_nodes.insert(task_id, node_id);
                Ok(node_id)
            }
        }
    }
}