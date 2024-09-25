use gosub_shared::traits::layouter::Layouter;
use gosub_shared::traits::render_tree::HasRenderTree;

pub struct MyLayouter;

impl<C: HasRenderTree> Layouter<C> for MyLayouter {
    fn do_layouter_things(&self) {
        println!("Doing layouter things");
    }

    fn new() -> Self {
        Self
    }
}