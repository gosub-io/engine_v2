use gosub_shared::node_id::NodeId;
use gosub_shared::traits::css_system::{CssSystem, HasCssSystem};
use gosub_shared::traits::document::{Document};
use gosub_shared::traits::node::{Node as _};
use crate::node::arena::NodeArena;
use crate::node::node_impl::Node;

pub struct MyDocument<C: HasCssSystem> {
    arena: NodeArena<Node>,
    url: String,
    _marker: std::marker::PhantomData<C>,
}

// impl<C: HasCssSystem> HasDocument for MyDocument<C> {
//     type Document = MyDocument<C::CssSystem>;
//     type Node = Node;
// }


// impl<C: HasCssSystem> Document<MyDocument<C>> for MyDocument<C> {
//     fn do_doc_things_with_css(&self, css: Self::CssSystem) {
//         todo!()
//     }
//     fn do_document_things(&self) {
//         todo!()
//     }
//     fn get_node(&self, id: NodeId) -> Option<&Self::Node> {
//         todo!()
//     }
//     fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Self::Node> {
//         todo!()
//     }
//     fn get_root_node(&self) -> Option<&Self::Node> {
//         todo!()
//     }
//     fn new(url: &str) -> Self {
//         todo!()
//     }
//     fn register_node_at(&mut self, node: Self::Node, parent_id: NodeId, position: Option<usize>) -> NodeId {
//         todo!()
//     }
//
//
// }

impl<C: HasCssSystem> HasCssSystem for MyDocument<C> {
    type CssSystem = C::CssSystem;
}

// impl<C: HasCssSystem> HasNode for MyDocument<C> {
//     type Node = Node;
//     // type NodeBuilder = NodeBuilder<Self::Node>;
// }

impl<C: HasCssSystem> Document<C> for MyDocument<C> {
    type Node = Node;

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

    fn get_url(&self) -> &str {
        self.url.as_str()
    }
}
