use gosub_shared::node_id::NodeId;
use gosub_shared::traits::node::Node;
use std::collections::HashMap;

/// The node arena is used by our Document implementation to store nodes. This way we can easily
/// reference nodes by their ID, and we can also easily detach nodes from the arena if needed.
pub struct NodeArena<N: Node> {
    nodes: HashMap<NodeId, N>,
    next_node_id: u32,
}

impl<N: Node> NodeArena<N> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            next_node_id: 0,
        }
    }

    pub fn next_node_id(&mut self) -> NodeId {
        let id = NodeId::new(self.next_node_id);
        self.next_node_id += 1;

        id
    }

    pub fn add_node(&mut self, mut node: N) -> NodeId {
        // If a node is already registered, we should use that node-id instead of generating a new
        // one.
        let node_id = if node.id().is_some() {
            node.id().unwrap()
        } else {
            self.next_node_id()
        };

        node.register(node_id);
        self.nodes.insert(node_id, node);

        node_id
    }

    pub fn update_node(&mut self, node_id: NodeId, node: N) {
        self.nodes.insert(node_id, node);
    }

    pub fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut N> {
        self.nodes.get_mut(&node_id)
    }

    pub fn get_node(&self, node_id: NodeId) -> Option<&N> {
        self.nodes.get(&node_id)
    }

    pub fn detach_node(&mut self, node_id: NodeId) -> Option<N> {
        self.nodes.remove(&node_id)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::builder::NodeBuilder;
    use crate::node::node_impl::Node as NodeImpl;
    use gosub_shared::traits::node::NodeBuilder as _;

    #[test]
    fn test_add_node() {
        let mut arena: NodeArena<NodeImpl> = NodeArena::new();

        let node = NodeBuilder::new_text_node("Hello, world!");
        let node_id = arena.add_node(node);
        assert_eq!(node_id.id(), 0);

        let node = NodeBuilder::new_text_node("Goodbye!");
        let node_id = arena.add_node(node);
        assert_eq!(node_id.id(), 1);
        assert_eq!(arena.len(), 2);

        let node = arena.get_node(node_id).unwrap();
        assert_eq!(node.is_registered(), true);
        assert_eq!(node.id(), Some(node_id));
    }

    #[test]
    fn test_get_node() {
        let mut arena: NodeArena<NodeImpl> = NodeArena::new();

        let node = NodeBuilder::new_text_node("Hello, world!");
        let node_id = arena.add_node(node);
        let node = arena.get_node(node_id);
        assert!(node.is_some());
    }
}
