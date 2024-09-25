use gosub_shared::document::DocumentHandle;
use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::node::HasNode;

pub struct DocumentBuilder<C: HasDocument + HasNode> {
    _phantom: std::marker::PhantomData<C>,
}

impl <C: HasDocument + HasNode> DocumentBuilder<C> {
    pub fn new_document(url: &str) -> DocumentHandle<C> {
        let doc = C::Document::new(url);
        DocumentHandle::new(doc)
    }

    pub fn new_document_fragment(_context: C::Node) -> DocumentHandle<C> {
        todo!("not yet implemented")
    }
}