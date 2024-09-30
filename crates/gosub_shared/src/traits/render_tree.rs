use crate::traits::css_system::CssSystem;
use crate::traits::document::{Document, HasDocument};
use crate::traits::layouter::HasLayouter;

pub trait HasRenderTree: Sized + HasDocument {
    type RenderTree: RenderTree<Self::Document>;
}

pub trait RenderTree<C: HasDocument>: Sized + HasRenderTree {
    fn do_render_tree_things(&self, doc: &C::Document);

    fn new() -> Self;
}