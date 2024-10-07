use crate::document::document::MyDocument;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::css_system::HasCssSystem;
use gosub_shared::traits::document::HasDocument;
use gosub_shared::traits::node::{ElementData, Node as _};

pub(crate) enum MutationEvents {
    RegisterNode(NodeId),
    UpdateNode(NodeId),
    DetachNode(NodeId),
}

impl<C: HasCssSystem + HasDocument> MyDocument<C> {
    pub(crate) fn mutate_document(&mut self, event: MutationEvents) {
        match event {
            MutationEvents::RegisterNode(id) => {
                self.add_named_element(id);
            }
            MutationEvents::UpdateNode(id) => {
                self.add_named_element(id);
            }
            MutationEvents::DetachNode(id) => {
                self.remove_named_element(id);
            }
        }
    }

    fn add_named_element(&mut self, node_id: NodeId) {
        let Some(node) = self.arena.get_node(node_id) else {
            return;
        };

        let Some(element) = node.get_element_data() else {
            return;
        };

        if element.attributes().contains_key("id") {
            let id_name = element.attributes().get("id").unwrap();

            self.named_elements.insert(id_name.clone(), node_id);
        }
    }

    fn remove_named_element(&mut self, node_id: NodeId) {
        let Some(node) = self.arena.get_node(node_id) else {
            return;
        };

        let Some(_) = node.get_element_data() else {
            return;
        };

        self.named_elements.retain(|_, v| *v != node_id);
    }
}
