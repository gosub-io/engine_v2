use gosub_shared::traits::css_system::HasCssSystem;
use gosub_shared::traits::document::Document;

struct MyDocument;

impl<C: HasCssSystem> Document<C> for MyDocument {
    fn do_document_things(&self) {
        println!("Doing document things");
    }

    fn new() -> Self {
        Self
    }
}
