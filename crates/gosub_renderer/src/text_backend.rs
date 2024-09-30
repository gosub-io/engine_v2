use gosub_shared::traits::render_backend::{HasRenderBackend, RenderBackend};

pub struct MyTextRenderBackend;

impl HasRenderBackend for MyTextRenderBackend { type RenderBackend = Self; }

impl RenderBackend for MyTextRenderBackend {
    fn do_render_backend_things(&self) {
        println!("Doing TEXTMODE render backend things");
    }

    fn new() -> Self {
        Self
    }
}
