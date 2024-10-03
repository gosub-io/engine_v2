use crate::document::DocumentHandle;
use crate::traits::document::HasDocument;

pub trait HasRenderTree: Sized + HasDocument {
    type RenderTree: RenderTree<Self>;
}

pub trait RenderTree<C: HasDocument>: Sized {
    fn from_document(handle: DocumentHandle<C>) -> Self;
}