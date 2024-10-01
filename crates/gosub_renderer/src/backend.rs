use gosub_shared::traits::render_backend::{HasRenderBackend, RenderBackend};

pub struct MyRenderBackend;

impl HasRenderBackend for MyRenderBackend { type RenderBackend = Self; }

impl RenderBackend for MyRenderBackend {
    fn do_render_backend_things(&self) {
        println!("Doing render backend things");
    }

    fn new() -> Self {
        Self
    }
}
