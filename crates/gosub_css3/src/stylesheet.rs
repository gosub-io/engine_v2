use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum CssValue {
    Unit(f32, String),
    Keyword(String),
    ColorValue(String),
    List(Vec<CssValue>)
}

#[derive(Clone, Debug)]
pub struct CssDeclaration {
    name: String,
    value: CssValue,
    important: bool,
}

impl Display for CssValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CssValue::Unit(value, unit) => write!(f, "{}{}", value, unit),
            CssValue::Keyword(value) => write!(f, "{}", value),
            CssValue::ColorValue(value) => write!(f, "{}", value),
            CssValue::List(args) => {
                let res = args
                    .iter()
                    .map(|n| format!("{}", n))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}", res)
            }
        }
    }
}

impl CssDeclaration {
    pub fn new(name: &str, value: CssValue, important: bool) -> Self {
        Self {
            name: name.to_string(),
            value,
            important
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &CssValue {
        &self.value
    }

    pub fn important(&self) -> bool {
        self.important
    }
}

#[derive(Clone, Debug)]
pub struct CssRule {
    selectors: Vec<String>,
    declarations: Vec<CssDeclaration>
}

impl CssRule {
    pub fn new() -> Self {
        Self {
            selectors: Vec::new(),
            declarations: Vec::new()
        }
    }

    pub fn add_selector(&mut self, selector: &str) {
        self.selectors.push(selector.to_string());
    }

    pub fn add_declaration(&mut self, declaration: CssDeclaration) {
        self.declarations.push(declaration);
    }

    pub fn selectors(&self) -> &Vec<String> {
        &self.selectors
    }

    pub fn declarations(&self) -> &Vec<CssDeclaration> {
        &self.declarations
    }
}

#[derive(Clone, Debug)]
pub struct CssStylesheet {
    rules: Vec<CssRule>
}

impl gosub_shared::traits::css_system::CssStylesheet for CssStylesheet {
    fn new() -> Self {
        Self {
            rules: Vec::new()
        }
    }

    fn add_rule(&mut self, rule: CssRule) {
        self.rules.push(rule);
    }

    fn rules(&self) -> &Vec<CssRule> { &self.rules }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_value_display() {
        let value = CssValue::Unit(1.0, "px".to_string());
        assert_eq!(format!("{}", value), "1px");

        let value = CssValue::Keyword("solid".to_string());
        assert_eq!(format!("{}", value), "solid");

        let value = CssValue::ColorValue("black".to_string());
        assert_eq!(format!("{}", value), "black");

        let value = CssValue::List(vec![
            CssValue::Unit(1.0, "px".to_string()),
            CssValue::Keyword("solid".to_string()),
            CssValue::ColorValue("black".to_string())
        ]);
        assert_eq!(format!("{}", value), "1px, solid, black");
    }

    #[test]
    fn test_css_declaration() {
        let declaration = CssDeclaration::new("color", CssValue::ColorValue("red".to_string()), false);
        assert_eq!(declaration.name(), "color");
        assert_eq!(format!("{}", declaration.value()), "red");
        assert_eq!(declaration.important(), false);
    }

    #[test]
    fn test_css_rule() {
        let mut rule = CssRule::new();
        rule.add_selector("body");
        rule.add_declaration(CssDeclaration::new("color", CssValue::ColorValue("red".to_string()), false));
        assert_eq!(rule.selectors(), &vec!["body".to_string()]);
        assert_eq!(rule.declarations().len(), 1);
    }

    #[test]
    fn test_css_stylesheet() {
        let stylesheet = CssStylesheet::parse("body { doesnt really matter; }");
        assert_eq!(stylesheet.rules().len(), 2);
    }
}