use crate::traits::css_system::CssSystem;
use crate::traits::document::Document;
use crate::traits::layouter::{HasLayouter, Layouter};
use crate::traits::render_tree::RenderTree;

pub trait HasTreeDrawer: Sized + HasLayouter {
    type TreeDrawer: TreeDrawer<Self::Layouter>;
}


pub trait TreeDrawer<C: HasLayouter>: Sized + HasTreeDrawer {
    fn do_tree_drawer_things(&self);

    fn new() -> Self;
}