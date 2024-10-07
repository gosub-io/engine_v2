pub mod css3parser;
pub mod stylesheet;

use crate::stylesheet::{CssDeclaration, CssRule, CssStylesheet, CssValue};
use gosub_shared::traits::css_system::HasCssSystem;

pub struct MyCssSystem {
    pub dummy: u32,
    pub some_other_stuff: bool,
    pub and_a_vec: Vec<i32>,
    pub last_entry: CssValue,
}

impl HasCssSystem for MyCssSystem {
    type CssStylesheet = CssStylesheet;
    type CssRule = CssRule;
    type CssDeclaration = CssDeclaration;
    type CssValue = CssValue;
}
