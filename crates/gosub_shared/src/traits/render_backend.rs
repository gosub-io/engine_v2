use crate::traits::layouter::HasLayouter;

pub trait HasRenderBackend: Sized + HasLayouter {
    type RenderBackend: RenderBackend<Self>;
}

pub trait RenderBackend<C: HasLayouter>: Sized {
    fn from_layouter(layouter: C::Layouter) -> Self;
    fn render_scene(&mut self);
}
