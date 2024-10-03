use gosub_shared::document::DocumentHandle;
use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::node::Node;
use gosub_shared::node_id::NodeId;

pub struct TreeIterator<C: HasDocument> {
    stack: Vec<NodeId>,
    handle: DocumentHandle<C>,
}

impl <C: HasDocument> TreeIterator<C> {
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
            let node = binding.get_node(node_id).unwrap();
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

    #[test]
    fn test_tree_iterator() {
        let mut doc = MyDocument::new("test");
        let root = doc.get_root_node().unwrap();
        let body = doc.add_element(root.id(), "body", ElementData::new());
        let div = doc.add_element(body, "div", ElementData::new());
        let text = doc.add_text(div, "Hello, world!", TextData::new());
        let comment = doc.add_comment(div, "This is a comment", CommentData::new());
        let doctype = doc.add_doctype(root.id(), "html", DoctypeData::new());

        let mut iter = DocumentTreeIterator::new(doc.clone());
        assert_eq!(iter.next(), Some(root.id()));
        assert_eq!(iter.next(), Some(body));
        assert_eq!(iter.next(), Some(div));
        assert_eq!(iter.next(), Some(text));
        assert_eq!(iter.next(), Some(comment));
        assert_eq!(iter.next(), Some(doctype));
        assert_eq!(iter.next(), None);
    }
}