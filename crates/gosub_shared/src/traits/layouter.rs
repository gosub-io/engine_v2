use crate::traits::render_tree::HasRenderTree;

pub trait HasLayouter: Sized  + HasRenderTree {
    type Layouter: Layouter<Self>;
}

pub trait Layouter<C: HasRenderTree>: Sized {
    fn do_layouter_things(&self);

    fn new() -> Self;
}