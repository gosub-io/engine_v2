use gosub_shared::traits::layouter::HasLayouter;
use gosub_shared::traits::render_backend::RenderBackend;

/// Vello is a 2d engine that utilizes the GPU to render the layout. This backend will take the
/// layouter, and render the layout using the Vello engine.

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
