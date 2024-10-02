pub mod stylesheet;
pub mod css3parser;

use gosub_shared::traits::css_system::HasCssSystem;
use crate::stylesheet::{CssDeclaration, CssRule, CssStylesheet, CssValue};

pub struct MyCssSystem;

impl HasCssSystem for MyCssSystem {
    type CssStylesheet = CssStylesheet;
    type CssRule = CssRule;
    type CssDeclaration = CssDeclaration;
    type CssValue = CssValue;
}

