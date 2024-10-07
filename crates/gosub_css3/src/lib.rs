pub mod stylesheet;
pub mod css3parser;

use gosub_shared::traits::css_system::HasCssSystem;
use crate::stylesheet::{CssDeclaration, CssRule, CssStylesheet, CssValue};

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

