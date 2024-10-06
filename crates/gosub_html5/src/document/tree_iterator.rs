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
    use gosub_css3::MyCssSystem;
    use gosub_shared::traits::node::{CommentData, DocTypeData, ElementData, TextData};
    use crate::document::builder::DocumentBuilder;
    use crate::document::document::MyDocument;
    use crate::document::tree_iterator::TreeIterator;

    #[test]
    fn test_tree_iterator() {
        let mut handle = DocumentBuilder::<MyDocument<MyCssSystem>>::new_document("test://test");

        let binding = handle.clone().get();
        let root = binding.get_root().unwrap();
        let body = binding.add_element(root.id(), "body", ElementData::new("body", "html"));
        let div = binding.add_element(body, "div", ElementData::new("div", "html"));
        let text = binding.add_text(div, "Hello, world!", TextData::new("Hello world!"));
        let comment = binding.add_comment(div, "This is a comment", CommentData::new("This is a comment"));
        let doctype = binding.add_doctype(root.id(), "html", DocTypeData::new("foo", "public", "system"));

        let mut iter = TreeIterator::new(handle.clone());
        assert_eq!(iter.next(), Some(root.id()));
        assert_eq!(iter.next(), Some(body));
        assert_eq!(iter.next(), Some(div));
        assert_eq!(iter.next(), Some(text));
        assert_eq!(iter.next(), Some(comment));
        assert_eq!(iter.next(), Some(doctype));
        assert_eq!(iter.next(), None);
    }
}