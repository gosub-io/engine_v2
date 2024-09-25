use gosub_shared::traits::renderer::{HasRenderTree, Layouter};

struct MyLayouter;

impl<C: HasRenderTree> Layouter<C> for MyLayouter {
    fn do_layouter_things(&self) {
        println!("Doing layouter things");
    }

    fn new() -> Self {
        Self
    }
}