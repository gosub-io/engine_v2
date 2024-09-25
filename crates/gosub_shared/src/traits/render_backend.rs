pub trait RenderBackend: Sized {
    fn do_render_backend_things(&self);

    fn new() -> Self;
}