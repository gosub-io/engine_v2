use crate::traits::css_system::CssSystem;
use crate::traits::document::{Document, HasDocument};
use crate::traits::layouter::HasLayouter;

pub trait HasRenderTree: Sized {
    type CssSystem: CssSystem;
    type Document: Document<Self>;
    type RenderTree: RenderTree<Self>;
}

impl<RT: HasRenderTree> HasDocument for RT {
    type CssSystem = RT::CssSystem;
    type Document = RT::Document;
}

impl<HL: HasLayouter> HasRenderTree for HL {
    type CssSystem = HL::CssSystem;
    type Document = HL::Document;
    type RenderTree = HL::RenderTree;
}

pub trait RenderTree<C: HasDocument>: Sized {
    fn do_render_tree_things(&self, doc: &C::Document);

    fn new() -> Self;
}