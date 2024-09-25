use gosub_shared::traits::document::HasDocument;
use gosub_shared::traits::renderer::RenderTree;

struct MyRenderTree;

impl<C: HasDocument> RenderTree<C> for MyRenderTree {
    fn do_render_tree_things(&self) {
        println!("Doing render tree things");
    }

    fn new() -> Self {
        Self
    }
}