use crate::traits::document::Document;
use crate::traits::document::HasDocument;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[derive(Debug)]
pub struct DocumentHandle<C: HasDocument>(pub Rc<RefCell<C::Document>>);

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