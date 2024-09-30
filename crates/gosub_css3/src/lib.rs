mod stylesheet;

use gosub_shared::traits::css_system::CssSystem;

pub struct MyCssSystem;

impl CssSystem for MyCssSystem {
    fn do_css_things(&self) {
        println!("Doing CSS things");
    }

    fn new() -> Self {
        Self
    }

    fn parse_stylesheet(&self, stylesheet: &str) -> CssStylesheet{
        println!("Parsing stylesheet: {}", stylesheet);
    }
}
