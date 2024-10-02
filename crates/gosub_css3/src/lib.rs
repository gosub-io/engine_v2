#[allow(dead_code)]
pub mod stylesheet;
pub mod parser;

use gosub_shared::traits::css_system::HasCssSystem;

pub struct MyCssSystem;

impl HasCssSystem for MyCssSystem { type CssSystem = Self; }

