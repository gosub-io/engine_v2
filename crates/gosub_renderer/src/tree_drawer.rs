use gosub_shared::traits::layouter::HasLayouter;
use gosub_shared::traits::tree_drawer::{TreeDrawer};

pub struct MyTreeDrawer<C: HasLayouter> {
    _marker: std::marker::PhantomData<C>,
}

// impl<C: HasLayouter> HasTreeDrawer for MyTreeDrawer<C> { type TreeDrawer = MyTreeDrawer<C::Layouter>; }
//
// impl<C: HasLayouter> HasLayouter for MyTreeDrawer<C> { type Layouter = C::Layouter; }
//
// impl<C: HasLayouter> HasRenderTree for MyTreeDrawer<C> { type RenderTree = C::RenderTree; }
//
// impl<C: HasLayouter> HasDocument for MyTreeDrawer<C> { type Document = C::Document; }
//
// impl<C: HasLayouter> HasCssSystem for MyTreeDrawer<C> { type CssSystem = C::CssSystem; }

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
