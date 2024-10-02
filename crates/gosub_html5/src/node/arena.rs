use std::collections::HashMap;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::node::Node;

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

    pub fn add_node(&mut self, mut node: N) -> NodeId {
        let id = NodeId::new(self.next_node_id);
        self.next_node_id += 1;

        // we mutate ID here
        node.register(id);

        // But we "move" node into the self.nodes.. so node is not an owner anymore. Does this work with mut?
        self.nodes.insert(id, node);

        id
    }

    pub fn get_node(&self, node_id: NodeId) -> Option<&N> {
        self.nodes.get(&node_id)
    }

    pub fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut N> {
        self.nodes.get_mut(&node_id)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use gosub_shared::traits::node::NodeBuilder as _;
    use super::*;
    use crate::node::builder::NodeBuilder;
    use crate::node::node_impl::Node as NodeImpl;

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

    #[test]
    fn test_get_node_mut() {
        let mut arena: NodeArena<NodeImpl> = NodeArena::new();

        let node = NodeBuilder::new_text_node("Hello, world!");
        let node_id = arena.add_node(node);

        let node = arena.get_node_mut(node_id);
        assert!(node.is_some());
    }
}