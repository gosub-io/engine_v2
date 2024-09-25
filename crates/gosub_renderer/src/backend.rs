use gosub_shared::traits::renderer::RenderBackend;

struct MyRenderBackend;

impl RenderBackend for MyRenderBackend {
    fn do_render_backend_things(&self) {
        println!("Doing render backend things");
    }

    fn new() -> Self {
        Self
    }
}
