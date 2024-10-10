use std::collections::HashMap;
use gosub_shared::document::DocumentHandle;
use gosub_shared::location::Location;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::node::{Node, NodeBuilder as _};
use crate::node::builder::NodeBuilder;


#[derive(Copy)]
pub struct TaskId(usize);

pub enum TaskDestination {
    Node(NodeId, Option<usize>),
    Task(TaskId, Option<usize>),
    DocumentRoot(Option<usize>),
}

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

pub enum DocumentTask {
    CreateElement {
        name: String,
        namespace: String,
        location: Location
    },
    CreateText {
        content: String,
        location: Location
    },
    CreateComment {
        content: String,
        location: Location
    },
    InsertAttribute {
        key: String,
        value: String,
        location: Location
    }
}

pub struct DocumentTaskQueue<C: HasDocument> {    
    doc_handle: DocumentHandle<C>,
    pending_tasks: Vec<Task>,
    next_task_id: usize,
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

    pub fn has_pending_tasks(&self) -> bool {
        !self.pending_tasks.is_empty()
    }

    pub fn add_task(&mut self, task: DocumentTask, destination: TaskDestination) -> TaskId {
        self.next_task_id += 1;
        let task_id = TaskId(self.next_task_id);

        self.pending_tasks.push(Task::new(task_id, task, destination));

        task_id
    }

    pub fn flush(&mut self) -> Vec<String> {
        let mut errors = Vec::new();

        for current_task in self.pending_tasks {
            match current_task.task {
                DocumentTask::CreateElement { name, namespace, location: _location } => {
                    let new_node = NodeBuilder::new_element_node(name.into(), namespace.into());
                    self.insert_node(new_node, current_task.destination, current_task.task_id);
                }
                DocumentTask::CreateText { content, location: _location } => {
                    let new_node = NodeBuilder::new_text_node(content.into());
                    self.insert_node(new_node, current_task.destination, current_task.task_id);
                }
                DocumentTask::CreateComment { content, location: _location } => {
                    let new_node = NodeBuilder::new_comment_node(content.into());
                    self.insert_node(new_node, current_task.destination, current_task.task_id);
                }
                DocumentTask::InsertAttribute { key, value, location: _location } => {
                    // if let Some(node) = binding.detach_node(*node_id) {
                    //     if !node.is_element() {
                    //         binding.update_node(node_id, node);
                    //
                    //         errors.push(format!("Node with id {} is not an element node", node_id));
                    //         continue;
                    //     }
                    //
                    //     node.insert_attribute(key, value);
                    //     binding.update_node(node_id, node);
                    // } else {
                    //     errors.push(format!("Node with id {} not found", node_id));
                    // }
                }
            }
        }

        self.pending_tasks.clear();
        self.task_nodes.clear();

        errors
    }

    fn insert_node(&mut self, node: C::Node, destination: TaskDestination, task_id: TaskId) {
        let mut binding = self.doc_handle.get_mut();

        match destination {
            TaskDestination::Node(parent_id, position) => {
                let node_id = binding.register_node_at(node, parent_id, position);
                self.task_nodes.insert(task_id, node_id);
            }
            TaskDestination::Task(task_id, position) => {
                binding.register_node_at(node, self.task_nodes[task_id], position);
            }
            TaskDestination::DocumentRoot(position) => {
                binding.register_node_at(node, NodeId::root(), position);
            }
        }
    }
}