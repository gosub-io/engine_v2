use gosub_shared::node_id::NodeId;
use gosub_shared::traits::css_system::HasCssSystem;
use gosub_shared::traits::document::{Document};
use gosub_shared::traits::node::{Node as _, NodeBuilder as _};
use crate::node::arena::NodeArena;
use crate::node::builder::NodeBuilder;
use crate::node::node_impl::Node;

pub struct MyDocument<C: HasCssSystem> {
    arena: NodeArena<Node>,
    url: String,
    _marker: std::marker::PhantomData<C>,
}

impl<C: HasCssSystem> HasCssSystem for MyDocument<C> {
    type CssSystem = C::CssSystem;
}

impl<C: HasCssSystem> Document<C> for MyDocument<C> {
    type Node = Node;

    fn new(url: &str) -> Self {
        let mut doc = Self {
            arena: NodeArena::new(),
            url: url.into(),
            _marker: std::marker::PhantomData,
        };

        // Create initial root node
        let root_node = NodeBuilder::new_document_node();
        doc.arena.add_node(root_node);

        doc
    }

    fn register_node_at(&mut self, node: Self::Node, parent_id: NodeId, position: Option<usize>) -> NodeId {
        let node_id = self.arena.add_node(node);

        // Update the parent node to make sure the nodeid is added to the parent
        let mut parent_node = self.arena.detach_node(parent_id).unwrap();
        parent_node.add_child_at_position(node_id, position);
        self.arena.update_node(parent_id, parent_node);

        node_id
    }

    fn get_root_node(&self) -> Option<&Self::Node> {
        self.arena.get_node(NodeId::root())
    }

    fn get_node_clone(&self, id: NodeId) -> Option<Self::Node> {
        self.arena.get_node(id).map(|n| n.clone())
    }

    fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Self::Node> {
        self.arena.get_node_mut(id)
    }

    fn get_node(&self, id: NodeId) -> Option<&Self::Node> {
        self.arena.get_node(id)
    }

    fn update_node(&mut self, id: NodeId, node: Self::Node) {
        self.arena.update_node(id, node);
    }

    fn detach_node(&mut self, id: NodeId) -> Option<Self::Node> {
        self.arena.detach_node(id)
    }

    fn attach_node(&mut self, id: NodeId, node: Self::Node) {
        self.arena.update_node(id, node);
    }

    fn get_url(&self) -> &str {
        self.url.as_str()
    }
}
