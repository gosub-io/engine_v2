use crate::traits::document::Document;
use crate::traits::document::HasDocument;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

pub struct DocumentHandle<C: HasDocument>(pub Rc<RefCell<C::Document>>);

impl<C: HasDocument> Debug for DocumentHandle<C>
where
    C::Document: Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DocumentHandle")
            .field(&self.0)
            .finish()
    }
}


impl<C: HasDocument> DocumentHandle<C> {
    /// Creates a new document handle based on the given document. The document itself can be
    /// referenced hereafter with the `get` or `get_mut()` methods. Note that the document handle
    /// is also stored within the document, in case of functionality within the document that
    /// relies on calling other functionality with a document-handle (for instance,
    /// the `QueryProcessor` in `query()`).
    pub fn new(document: C::Document) -> Self {
        let handle = Self(Rc::new(RefCell::new(document)));

        let mut binding = handle.clone();
        binding.get_mut().set_handle(handle.clone());

        handle
    }

    /// Returns a reference to the document.
    pub fn get(&self) -> Ref<'_, C::Document> {
        self.0.borrow()
    }

    /// Returns a mutable reference to the document.
    pub fn get_mut(&mut self) -> RefMut<'_, C::Document> {
        RefCell::borrow_mut(&self.0)
    }
}

impl<C: HasDocument> Clone for DocumentHandle<C> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<C: HasDocument> PartialEq for DocumentHandle<C> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::mock::{MockConfig, MockDocument};
    use super::*;

    #[test]
    fn test_dochandle() {
        let mock_doc = MockDocument {
            content: "Hello".into(),
            handle: None,
        };
        let mut doc_handle: DocumentHandle<MockConfig> = DocumentHandle::new(mock_doc);
        assert_eq!(doc_handle.get().content, "Hello");

        doc_handle.get_mut().content = "World".into();
        assert_eq!(doc_handle.get().content, "World");

        let cloned_handle = doc_handle.clone();
        assert_eq!(cloned_handle.get().content, "World");
        assert_eq!(doc_handle, cloned_handle);

        let mock_doc = MockDocument {
            content: "World".into(),
            handle: None,
        };
        let doc2_handle = DocumentHandle::new(mock_doc);

        assert_ne!(doc_handle, doc2_handle);
    }
}