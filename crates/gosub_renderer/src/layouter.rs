use gosub_shared::traits::layouter::{Layouter};
use gosub_shared::traits::render_tree::HasRenderTree;

pub struct MyLayouter<C: HasRenderTree> {
    _marker: std::marker::PhantomData<C>,
}

impl<C: HasRenderTree> Layouter<C> for MyLayouter<C> {
    fn do_layouter_things(&self) {
        println!("Doing layouter things");
    }

    fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}