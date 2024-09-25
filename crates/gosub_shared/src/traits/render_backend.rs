pub trait HasRenderBackend {
    type RenderBackend: RenderBackend;
}

pub trait RenderBackend: Sized {
    fn do_render_backend_things(&self);

    fn new() -> Self;
}

impl<B: RenderBackend> HasRenderBackend for B {
    type RenderBackend = B;
}
