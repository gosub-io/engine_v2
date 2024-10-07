use crate::traits::document::Document;
use crate::traits::document::HasDocument;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[derive(Debug)]
pub struct DocumentHandle<D: HasDocument>(pub Rc<RefCell<D>>);

impl<D: HasDocument> DocumentHandle<D> {
    pub fn new(document: D) -> Self {
        let handle = Self(Rc::new(RefCell::new(document)));

        let mut binding = handle.clone().get_mut();
        binding.set_handle(handle.clone());

        handle
    }

    pub fn get(&self) -> Ref<'_, D::Document> {
        self.0.borrow()
    }

    pub fn get_mut(&mut self) -> RefMut<'_, D::Document> {
        RefCell::borrow_mut(&self.0)
    }
}

impl<D: HasDocument> Clone for DocumentHandle<D> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<D: HasDocument> PartialEq for DocumentHandle<D> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDocument {
        content: String,
    }

    struct Mock;

    impl HasDocument for Mock {
        type Document = MockDocument;
    }

    #[test]
    fn test_dochandle() {
        let mock_doc = MockDocument {
            content: "Hello".into(),
        };
        let doc_handle = DocumentHandle::new(mock_doc);

        assert_eq!(doc_handle.get().content, "Hello");

        doc_handle.get_mut().content = "World".into();
        assert_eq!(doc_handle.get().content, "World");

        let cloned_handle = doc_handle.clone();
        assert_eq!(cloned_handle.get().content, "World");
        assert_eq!(doc_handle, cloned_handle);

        let mock_doc = MockDocument {
            content: "Hello".into(),
        };
        let doc2_handle = DocumentHandle::new(mock_doc);
        assert_ne!(doc_handle, doc2_handle);
    }
}
