use gosub_shared::document::DocumentHandle;
use gosub_shared::traits::document::{Document, HasDocument};

/// This struct can be used to create new documents. It should be used in favor of building
/// documents directly.

pub struct DocumentBuilder<C: HasDocument> {
    _phantom: std::marker::PhantomData<C>,
}

impl<C: HasDocument> DocumentBuilder<C> {
    pub fn new_document(url: &str) -> DocumentHandle<C> {
        let doc = C::Document::new(url);

        DocumentHandle::new(doc)
    }

    pub fn new_document_fragment(_context: C::Node) -> DocumentHandle<C> {
        todo!("not yet implemented")
        // let doc = C::Document::new_with_rootnode(_context);
        // DocumentHandle::new(doc);
    }
}
