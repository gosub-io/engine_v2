use gosub_shared::traits::layouter::HasLayouter;
use gosub_shared::traits::render_backend::RenderBackend;

/// A textual backend renderer would render the layout to a text-based output. This could be a
/// terminal, a file, or any other text-based output.

pub struct MyTextRenderBackend<C: HasLayouter> {
    _marker: std::marker::PhantomData<C>,
}

impl<C: HasLayouter> RenderBackend<C> for MyTextRenderBackend<C> {
    fn from_layouter(_layouter: C::Layouter) -> Self {
        todo!()
    }

    fn render_scene(&mut self) {
        todo!()
    }
}
