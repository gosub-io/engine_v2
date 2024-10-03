use crate::document::DocumentHandle;
use crate::node_id::NodeId;
use crate::traits::document::HasDocument;

pub trait HasRenderTree: Sized + HasDocument {
    type RenderTree: RenderTree<Self>;
}

pub trait RenderTree<C: HasDocument>: Sized {
    fn from_document(handle: DocumentHandle<C>) -> Self;

    fn get_property(&self, node_id: NodeId, prop_name: &str) -> Option<&Vec<C::CssDeclaration>>;
    fn get_properties(&self, node_id: NodeId) -> Option<&Vec<C::CssDeclaration>>;
}