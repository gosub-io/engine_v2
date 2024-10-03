use gosub_shared::traits::render_backend::{HasRenderBackend, RenderBackend};

pub struct MyVelloRenderBackend;

impl HasRenderBackend for MyVelloRenderBackend { type RenderBackend = Self; }

impl RenderBackend for MyVelloRenderBackend {
    fn do_render_backend_things(&self) {
        println!("Doing render backend things for VELLO 2d system");
    }

    fn new() -> Self {
        Self
    }
}
