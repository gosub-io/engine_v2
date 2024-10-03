use gosub_shared::traits::layouter::Layouter;
use gosub_shared::traits::render_tree::HasRenderTree;

pub struct MyLayouter<C: HasRenderTree> {
    _layout_stuff: i32,
    _marker: std::marker::PhantomData<C>,
}

impl<C: HasRenderTree> Layouter<C> for MyLayouter<C> {
    fn from_render_tree(_render_tree: C::RenderTree) -> Self {
        Self {
            _layout_stuff: 2,
            _marker: std::marker::PhantomData,
        }
    }
}