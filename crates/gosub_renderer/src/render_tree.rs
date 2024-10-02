use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::render_tree::{RenderTree};

pub struct MyRenderTree<C: HasDocument> {
    _marker: std::marker::PhantomData<C>,
}

impl<C: HasDocument> RenderTree<C> for MyRenderTree<C> {
    fn do_render_tree_things(&self, doc: &C::Document) {
        doc.do_document_things();
        println!("Doing render tree things");
    }

    fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}