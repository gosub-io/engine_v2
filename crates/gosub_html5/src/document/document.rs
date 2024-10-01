use gosub_shared::node_id::NodeId;
use gosub_shared::traits::css_system::{CssSystem, HasCssSystem};
use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::node::{HasNode, Node as _};
use crate::node::arena::NodeArena;

pub struct MyDocument<C: HasCssSystem, N: HasNode> {
    arena: NodeArena<N>,
    url: String,
    _marker: std::marker::PhantomData<C>,
}

impl<C: HasCssSystem, N: HasNode> HasDocument for MyDocument<C, N> {
    type Document = MyDocument<C::CssSystem, N::Node>;
}

impl<C: HasCssSystem, N: HasNode> HasCssSystem for MyDocument<C, N> {
    type CssSystem = C::CssSystem;
}

impl<C: HasCssSystem, N: HasNode> HasNode for MyDocument<C, N> {
    type Node = N::Node;
    // type NodeBuilder = NodeBuilder<Self::Node>;
}

impl<C: HasCssSystem, N: HasNode> Document<C, N> for MyDocument<C, N> {
    // type Node = N::Node;

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

    fn do_doc_things_with_css(&self, css: C::CssSystem) {
        css.do_css_things();
    }

    fn do_document_things(&self) {
        todo!()
    }
}
