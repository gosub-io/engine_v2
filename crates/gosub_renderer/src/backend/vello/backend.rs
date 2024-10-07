use gosub_shared::traits::layouter::HasLayouter;
use gosub_shared::traits::render_backend::RenderBackend;

pub struct MyVelloRenderBackend<C: HasLayouter> {
    _marker: std::marker::PhantomData<C>,
}

impl<C: HasLayouter> RenderBackend<C> for MyVelloRenderBackend<C> {
    fn from_layouter(_layouter: C::Layouter) -> Self {
        todo!()
    }

    fn render_scene(&mut self) {
        todo!()
    }
}
