use std::collections::HashMap;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::css_system::HasCssSystem;
use gosub_shared::traits::document::{Document};
use gosub_shared::traits::node::{Node as _, NodeBuilder as _};
use crate::document::document_events::MutationEvents;
use crate::node::arena::NodeArena;
use crate::node::builder::NodeBuilder;
use crate::node::node_impl::Node;

pub struct MyDocument<C: HasCssSystem> {
    pub(crate) arena: NodeArena<Node>,
    url: String,
    stylesheets: Vec<C::CssStylesheet>,
    pub(crate) named_elements: HashMap::<String, NodeId>,
    _marker: std::marker::PhantomData<C>,
}

impl<C: HasCssSystem> HasCssSystem for MyDocument<C> {
    type CssStylesheet = C::CssStylesheet;
    type CssRule = C::CssRule;
    type CssDeclaration = C::CssDeclaration;
    type CssValue = C::CssValue;
}

impl<C: HasCssSystem> Document<C> for MyDocument<C> {
    type Node = Node;

    fn new(url: &str) -> Self {
        let mut doc = Self {
            arena: NodeArena::new(),
            url: url.into(),
            stylesheets: Vec::new(),
            named_elements: HashMap::new(),
            _marker: std::marker::PhantomData,
        };

        // Create initial root node
        let root_node = NodeBuilder::new_document_node();
        doc.arena.add_node(root_node);

        doc
    }

    fn register_node_at(&mut self, node: Self::Node, parent_id: NodeId, position: Option<usize>) -> NodeId {
        let node_id = self.arena.add_node(node);

        // Update document
        self.mutate_document(MutationEvents::RegisterNode(node_id));

        // Update the parent node to make sure the nodeid is added to the parent
        let mut parent_node = self.arena.detach_node(parent_id).unwrap();
        parent_node.add_child_at_position(node_id, position);
        self.arena.update_node(parent_id, parent_node);

        node_id
    }

    fn get_root_node(&self) -> Option<&Self::Node> {
        self.arena.get_node(NodeId::root())
    }

    fn get_node_clone(&self, node_id: NodeId) -> Option<Self::Node> {
        self.arena.get_node(node_id).map(|n| n.clone())
    }

    fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut Self::Node> {
        self.arena.get_node_mut(node_id)
    }

    fn get_node(&self, node_id: NodeId) -> Option<&Self::Node> {
        self.arena.get_node(node_id)
    }

    fn detach_node(&mut self, node_id: NodeId) -> Option<Self::Node> {
        self.mutate_document(MutationEvents::DetachNode(node_id));

        self.arena.detach_node(node_id)
    }

    fn update_node(&mut self, node_id: NodeId, node: Self::Node) {
        self.arena.update_node(node_id, node);

        self.mutate_document(MutationEvents::UpdateNode(node_id));
    }

    fn get_url(&self) -> &str {
        self.url.as_str()
    }

    fn stylesheets(&self) -> &Vec<C::CssStylesheet> {
        &self.stylesheets
    }

    fn add_stylesheet(&mut self, stylesheet: C::CssStylesheet) {
        self.stylesheets.push(stylesheet);
    }

    fn get_node_by_element_id(&self, name: &str) -> Option<NodeId> {
        self.named_elements.get(name).cloned()
    }
}
