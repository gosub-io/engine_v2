use crate::traits::css_system::CssSystem;
use crate::traits::document::Document;
use crate::traits::render_tree::{HasRenderTree, RenderTree};

pub trait HasLayouter: Sized {
    type Document: Document;
    type CssSystem: CssSystem;
    type RenderTree: RenderTree<Self>;
    type Layouter: Layouter<Self>;
}

pub trait Layouter<C: HasRenderTree>: Sized {
    fn do_layouter_things(&self);

    fn new() -> Self;
}