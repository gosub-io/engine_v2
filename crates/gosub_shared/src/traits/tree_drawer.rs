use crate::traits::css_system::CssSystem;
use crate::traits::document::Document;
use crate::traits::layouter::{HasLayouter, Layouter};
use crate::traits::render_tree::RenderTree;

pub trait HasTreeDrawer: Sized {
    type Document: Document<Self>;
    type CssSystem: CssSystem;
    type RenderTree: RenderTree<Self>;
    type Layouter: Layouter<Self>;
    type TreeDrawer: TreeDrawer<Self>;
}

impl<TD: HasTreeDrawer> HasLayouter for TD {
    type Document = TD::Document;
    type CssSystem = TD::CssSystem;
    type RenderTree = TD::RenderTree;
    type Layouter = TD::Layouter;
}

pub trait TreeDrawer<C: HasLayouter>: Sized {
    fn do_tree_drawer_things(&self);

    fn new() -> Self;
}