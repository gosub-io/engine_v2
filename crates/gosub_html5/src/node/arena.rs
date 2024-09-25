use std::collections::HashMap;
use gosub_shared::node_id::NodeId;
use crate::node::node_impl::Node;

pub struct NodeArena {
    nodes: HashMap<NodeId, Node>,
    next_node_id: u32,
}

impl NodeArena {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            next_node_id: 0,
        }
    }

    pub fn add_node(&mut self, mut node: Node) -> NodeId {
        let id = NodeId::new(self.next_node_id);
        self.next_node_id += 1;

        node.register(id);
        self.nodes.insert(id, node);
        id
    }

    pub fn get_node(&self, node_id: NodeId) -> Option<&Node> {
        self.nodes.get(&node_id)
    }

    pub fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(&node_id)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut arena = NodeArena::new();
        let node = Node::new_text_node("Hello, world!".to_string());
        let id = arena.add_node(node);
        assert_eq!(id.id(), 0);

        let node = Node::new_text_node("Hello, world!".to_string());
        let id = arena.add_node(node);
        assert_eq!(id.id(), 1);
        assert_eq!(arena.len(), 2);

        let node = arena.get_node(id).unwrap();
        assert_eq!(node.is_registered(), true);
        assert_eq!(node.id, Some(id));
    }

    #[test]
    fn test_get_node() {
        let mut arena = NodeArena::new();
        let node = Node::new_text_node("Hello, world!".to_string());
        let id = arena.add_node(node);
        let node = arena.get_node(id);
        assert!(node.is_some());
    }

    #[test]
    fn test_get_node_mut() {
        let mut arena = NodeArena::new();
        let node = Node::new_text_node("Hello, world!".to_string());
        let id = arena.add_node(node);
        let node = arena.get_node_mut(id);
        assert!(node.is_some());
    }
}