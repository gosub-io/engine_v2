use gosub_shared::document::DocumentHandle;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::node::Node;

pub struct TreeIterator<C: HasDocument> {
    stack: Vec<NodeId>,
    handle: DocumentHandle<C>,
}

impl<C: HasDocument> TreeIterator<C> {
    pub fn new(handle: DocumentHandle<C>) -> Self {
        Self {
            stack: vec![NodeId::root()],
            handle: handle.clone(),
        }
    }
}

impl<C: HasDocument> Iterator for TreeIterator<C> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node_id) = self.stack.pop() {
            let binding = self.handle.get();
            let node = binding.get_node(node_id)?;
            for child_id in node.children().iter().rev() {
                self.stack.push(*child_id);
            }
            Some(node_id)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::document::builder::DocumentBuilder;
    use crate::document::document::MyDocument;
    use crate::document::tree_iterator::TreeIterator;
    use crate::node::builder::NodeBuilder;
    use gosub_css3::MyCssSystem;
    use gosub_shared::node_id::NodeId;
    use gosub_shared::traits::node::NodeBuilder as _;

    #[test]
    fn test_tree_iterator() {
        let mut handle =
            DocumentBuilder::<MyDocument<MyCssSystem>>::new_document("https://example.com");

        let body_node = NodeBuilder::new_element_node("body", "html");
        let body_node_id = handle
            .get_mut()
            .register_node_at(body_node, NodeId::root(), None);

        let div_node = NodeBuilder::new_element_node("div", "html");
        let div_node_id = handle
            .get_mut()
            .register_node_at(div_node, body_node_id, None);

        let text_node = NodeBuilder::new_text_node("Hello, world!");
        let text_node_id = handle
            .get_mut()
            .register_node_at(text_node, div_node_id, None);

        let comment_node = NodeBuilder::new_comment_node("This is a comment");
        let comment_node_id = handle
            .get_mut()
            .register_node_at(comment_node, div_node_id, None);

        let doc_type = NodeBuilder::new_doctype_node("foo", "public", "system");
        let doctype_node_id = handle
            .get_mut()
            .register_node_at(doc_type, NodeId::root(), None);

        let mut iter = TreeIterator::<MyDocument<MyCssSystem>>::new(handle.clone());
        assert_eq!(iter.next().unwrap().id(), Some(NodeId::root()));
        assert_eq!(iter.next().unwrap().id(), Some(body_node_id));
        assert_eq!(iter.next().unwrap().id(), Some(div_node_id));
        assert_eq!(iter.next().unwrap().id(), Some(text_node_id));
        assert_eq!(iter.next().unwrap().id(), Some(comment_node_id));
        assert_eq!(iter.next().unwrap().id(), Some(doctype_node_id));
        assert_eq!(iter.next(), None);
    }
}
