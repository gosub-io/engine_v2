

pub trait HasRenderBackend {
    type RenderBackend: RenderBackend;
}


pub trait RenderBackend: Sized + HasRenderBackend {
    fn do_render_backend_things(&self);

    fn new() -> Self;
}