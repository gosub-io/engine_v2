use gosub_shared::traits::css_system::CssStylesheet as _;
use gosub_shared::traits::css_system::CssDeclaration as _;
use gosub_shared::traits::css_system::CssRule as _;
use gosub_shared::traits::css_system::CssValue as _;
use gosub_shared::traits::css_system::CssParser;
use gosub_shared::traits::css_system::HasCssSystem;
use crate::stylesheet::{CssDeclaration};

pub struct MyCss3Parser<C: HasCssSystem> {
    parser_state: u32,  // dummy parser state
    _marker: std::marker::PhantomData<C>,
}

impl<C: HasCssSystem> CssParser<C> for MyCss3Parser<C> {

    fn new() -> Self {
        Self {
            parser_state: 0,
            _marker: std::marker::PhantomData,
        }
    }

    /// Parse a CSS stylesheet. Will generate a mock stylesheet for now:
    ///
    /// ```css
    /// body {
    ///    color: red;
    /// }
    ///
    /// h1 {
    ///     border: 1px solid black;
    /// }
    /// ```
    fn parse_str(&mut self, _input: &str) -> C::CssStylesheet {
        let mut stylesheet = C::CssStylesheet::new();

        let mut rule = C::CssRule::new();
        rule.add_selector("body");
        rule.add_declaration(C::CssDeclaration::new(
            "color",
            C::CssValue::colorvalue("red"),
            false
        ));
        stylesheet.add_rule(rule);

        let mut rule = C::CssRule::new();
        rule.add_selector("h1");
        rule.add_declaration(C::CssDeclaration::new(
            "border",
            C::CssValue::list(vec![
                C::CssValue::unit(1.0, "px"),
                C::CssValue::keyword("solid"),
                C::CssValue::colorvalue("black")
            ]),
            false
        ));
        rule.add_declaration(C::CssDeclaration::new(
            "border-width",
            C::CssValue::unit(1.0, "px"),
            false
        ));
        rule.add_declaration(C::CssDeclaration::new(
            "border-style",
            C::CssValue::keyword("solid"),
            false
        ));
        rule.add_declaration(C::CssDeclaration::new(
            "border-color",
            C::CssValue::colorvalue("black"),
            false
        ));
        stylesheet.add_rule(rule);

        stylesheet
    }
}


#[cfg(test)]
mod tests {
    use crate::MyCssSystem;
    use super::*;

    #[test]
    fn test_css_stylesheet() {
        let mut parser = MyCss3Parser::<MyCssSystem>::new();

        let stylesheet = parser.parse_str("body { doesnt really matter; }");
        assert_eq!(stylesheet.rules().len(), 2);
    }
}