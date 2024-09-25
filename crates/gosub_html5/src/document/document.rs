use gosub_shared::node_id::NodeId;
use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::node::{HasNode, Node as _};
use crate::node::arena::NodeArena;
use crate::node::builder::NodeBuilder;

pub struct MyDocument<C: HasDocument> {
    arena: NodeArena<<C::Document as Document>::Node>,
    url: String,
    _marker: std::marker::PhantomData<C>,
}

impl<C: HasDocument + HasNode> Document for MyDocument<C> {
    type NodeBuilder = NodeBuilder<C>;
    type Node = <C::Document as Document>::Node;

    fn new(url: &str) -> Self {
        Self {
            arena: NodeArena::new(),
            url: url.into(),
            _marker: std::marker::PhantomData,
        }
    }

    fn register_node_at(&mut self, node: Self::Node, parent_id: NodeId, position: Option<usize>) -> NodeId {
        let node_id = self.arena.add_node(node);

        // Update the parent node to make sure the nodeid is added to the parent
        self.arena.get_node_mut(parent_id).unwrap().add_child_at_position(node_id, position);

        node_id
    }

    fn get_root_node(&self) -> Option<&Self::Node> {
        self.arena.get_node(NodeId::root())
    }

    fn get_node(&self, id: NodeId) -> Option<&Self::Node> {
        self.arena.get_node(id)
    }

    fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Self::Node> {
        self.arena.get_node_mut(id)
    }
}
