use gosub_shared::traits::css_system::{CssSystem, HasCssSystem};

pub struct MyCssSystem;

impl CssSystem for MyCssSystem {
    fn do_css_things(&self) {
        println!("Doing CSS things");
    }

    fn new() -> Self {
        Self
    }
}

impl HasCssSystem for MyCssSystem {
    type CssSystem = MyCssSystem;
}
