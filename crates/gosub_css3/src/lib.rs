use gosub_shared::traits::css_system::CssSystem;

pub struct MyCssSystem;

impl CssSystem for MyCssSystem {
    fn do_css_things(&self) {
        println!("Doing CSS things");
    }

    fn new() -> Self {
        Self
    }
}
