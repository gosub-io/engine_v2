use crate::traits::render_tree::HasRenderTree;

pub trait HasLayouter: Sized  + HasRenderTree {
    type Layouter: Layouter<Self>;
}

pub trait Layouter<C: HasRenderTree>: Sized {
    fn from_render_tree(render_tree: C::RenderTree) -> Self;
}