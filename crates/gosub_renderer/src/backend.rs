use gosub_shared::traits::render_backend::RenderBackend;

pub struct MyRenderBackend;

impl RenderBackend for MyRenderBackend {
    fn do_render_backend_things(&self) {
        println!("Doing render backend things");
    }

    fn new() -> Self {
        Self
    }
}
