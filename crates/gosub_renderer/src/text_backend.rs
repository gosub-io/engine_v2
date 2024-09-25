use gosub_shared::traits::render_backend::RenderBackend;

pub struct MyTextRenderBackend;

impl RenderBackend for MyTextRenderBackend {
    fn do_render_backend_things(&self) {
        println!("Doing TEXTMODE render backend things");
    }

    fn new() -> Self {
        Self
    }
}
