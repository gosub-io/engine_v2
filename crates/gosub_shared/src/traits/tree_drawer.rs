use crate::traits::layouter::HasLayouter;

pub trait HasTreeDrawer: Sized + HasLayouter {
    type TreeDrawer: TreeDrawer<Self::Layouter>;
}


pub trait TreeDrawer<C: HasLayouter>: Sized + HasTreeDrawer {
    fn do_tree_drawer_things(&self);

    fn new() -> Self;
}