use crate::traits::css_system::CssSystem;
use crate::traits::document::Document;
use crate::traits::render_tree::{HasRenderTree, RenderTree};

pub trait HasLayouter: Sized  + HasRenderTree {
    type Layouter: Layouter<Self::RenderTree>;
}

pub trait Layouter<C: HasRenderTree>: Sized + HasLayouter {
    fn do_layouter_things(&self);

    fn new() -> Self;
}