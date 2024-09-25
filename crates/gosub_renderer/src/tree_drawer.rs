use gosub_shared::traits::css_system::HasCssSystem;
use gosub_shared::traits::layouter::HasLayouter;
use gosub_shared::traits::tree_drawer::TreeDrawer;

pub struct MyTreeDrawer<C: HasLayouter> {
    _marker: std::marker::PhantomData<C>,
}

// impl<C: HasLayouter> HasTreeDrawer<C> for MyTreeDrawer<C> {
//     type TreeDrawer = Self;
// }
//
// impl<C: HasLayouter> HasLayouter<C> for MyTreeDrawer<C> {
//     type Layouter = C::Layouter;
// }
//
// impl<C: HasLayouter> HasRenderTree<C> for MyTreeDrawer<C> {
//     type RenderTree = C::RenderTree;
// }
//
// impl<C: HasLayouter> HasDocument<C> for MyTreeDrawer<C> {
//     type Document = C::Document;
// }
//
impl<C: HasLayouter> HasCssSystem for MyTreeDrawer<C> {
    type CssSystem = C::CssSystem;
}

impl<C: HasLayouter> TreeDrawer<C> for MyTreeDrawer<C> {
    fn do_tree_drawer_things(&self) {
        println!("Doing tree drawer things");
    }

    fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
