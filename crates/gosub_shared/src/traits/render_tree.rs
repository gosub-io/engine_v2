use crate::traits::document::HasDocument;

pub trait HasRenderTree: Sized + HasDocument {
    type RenderTree: RenderTree<Self>;
}

pub trait RenderTree<C: HasDocument>: Sized {
    fn do_render_tree_things(&self, doc: &C::Document);

    fn new() -> Self;
}
