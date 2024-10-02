#[allow(dead_code)]
mod stylesheet;

use gosub_shared::traits::css_system::{CssSystem, HasCssSystem};

pub struct MyCssSystem;

impl HasCssSystem for MyCssSystem { type CssSystem = Self; }

impl CssSystem for MyCssSystem {
    fn do_css_things(&self) {
        println!("Doing CSS things");
    }

    fn new() -> Self {
        Self
    }

    // fn parse_stylesheet(&self, stylesheet: &str) -> CssStylesheet{
    //     println!("Parsing stylesheet: {}", stylesheet);
    // }
}
