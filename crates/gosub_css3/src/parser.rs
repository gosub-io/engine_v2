use gosub_shared::traits::css_system::{CssDeclaration as _ , CssRule as _, CssStylesheet as _ };
use crate::stylesheet::{CssDeclaration, CssRule, CssStylesheet, CssValue};

pub struct CssParser;

impl gosub_shared::traits::css_system::CssParser for CssParser {
    type CssStylesheet = CssStylesheet;

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
    fn parse_str(_input: &str) -> Self::CssStylesheet {
        let mut stylesheet = CssStylesheet::new();

        let rule = CssRule::new();
        rule.add_selector("body");
        rule.add_declaration(CssDeclaration::new(
            "color",
            CssValue::ColorValue("red".into()),
            false
        ));
        stylesheet.add_rule(rule);

        let mut rule = CssRule::new();
        rule.add_selector("h1");
        rule.add_declaration(CssDeclaration::new(
            "border",
            CssValue::List(vec![
                CssValue::Unit(1.0, "px".to_string()),
                CssValue::Keyword("solid".to_string()),
                CssValue::ColorValue("black".to_string())
            ]),
            false
        ));
        rule.add_declaration(CssDeclaration::new(
            "border-width",
            CssValue::Unit(1.0, "px".to_string()),
            false
        ));
        rule.add_declaration(CssDeclaration::new(
            "border-style",
            CssValue::Keyword("solid".into()),
            false
        ));
        rule.add_declaration(CssDeclaration::new(
            "border-color",
            CssValue::ColorValue("black".into()),
            false
        ));
        stylesheet.add_rule(rule);

        stylesheet
    }
}