use gosub_shared::traits::css_system as css_traits;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum CssValue {
    Unit(f32, String),
    Keyword(String),
    ColorValue(String),
    List(Vec<CssValue>),
}

// impl Clone for CssValue {
//     fn clone(&self) -> Self {
//         match self {
//             CssValue::Unit(value, unit) => CssValue::Unit(*value, unit.clone()),
//             CssValue::Keyword(value) => CssValue::Keyword(value.clone()),
//             CssValue::ColorValue(value) => CssValue::ColorValue(value.clone()),
//             CssValue::List(args) => CssValue::List(args.clone())
//         }
//     }
// }

impl css_traits::CssValue for CssValue {
    fn unit(value: f32, unit: &str) -> Self {
        Self::Unit(value, unit.into())
    }

    fn keyword(value: &str) -> Self {
        Self::Keyword(value.into())
    }

    fn colorvalue(value: &str) -> Self {
        Self::ColorValue(value.into())
    }

    fn list(args: Vec<Self>) -> Self {
        Self::List(args)
    }

    fn is_unit(&self) -> bool {
        match self {
            CssValue::Unit(_, _) => true,
            _ => false,
        }
    }

    fn is_keyword(&self) -> bool {
        match self {
            CssValue::Keyword(_) => true,
            _ => false,
        }
    }

    fn is_color(&self) -> bool {
        match self {
            CssValue::ColorValue(_) => true,
            _ => false,
        }
    }

    fn is_list(&self) -> bool {
        match self {
            CssValue::List(_) => true,
            _ => false,
        }
    }
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

impl css_traits::HasCssSystem for CssDeclaration {
    type CssStylesheet = CssStylesheet;
    type CssRule = CssRule;
    type CssDeclaration = CssDeclaration;
    type CssValue = CssValue;
}

impl css_traits::CssDeclaration for CssDeclaration {
    fn new(name: &str, value: Self::CssValue, important: bool) -> Self {
        Self {
            name: name.to_string(),
            value,
            important,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn value(&self) -> CssValue {
        self.value.clone()
    }

    fn important(&self) -> bool {
        self.important
    }
}

#[derive(Clone, Debug)]
pub struct CssRule {
    selectors: Vec<String>,
    declarations: Vec<CssDeclaration>,
}

impl css_traits::HasCssSystem for CssRule {
    type CssStylesheet = CssStylesheet;
    type CssRule = CssRule;
    type CssDeclaration = CssDeclaration;
    type CssValue = CssValue;
}

impl css_traits::CssRule for CssRule {
    // type CssDeclaration = CssDeclaration;

    fn new() -> Self {
        Self {
            selectors: Vec::new(),
            declarations: Vec::new(),
        }
    }

    fn add_selector(&mut self, selector: &str) {
        self.selectors.push(selector.to_string());
    }

    fn add_declaration(&mut self, declaration: Self::CssDeclaration) {
        self.declarations.push(declaration);
    }

    fn selectors(&self) -> &Vec<String> {
        &self.selectors
    }

    fn declarations(&self) -> &Vec<Self::CssDeclaration> {
        &self.declarations
    }
}

#[derive(Clone, Debug)]
pub struct CssStylesheet {
    rules: Vec<CssRule>,
}

impl css_traits::HasCssSystem for CssStylesheet {
    type CssStylesheet = CssStylesheet;
    type CssRule = CssRule;
    type CssDeclaration = CssDeclaration;
    type CssValue = CssValue;
}

impl css_traits::CssStylesheet for CssStylesheet {
    // type CssRule = CssRule;

    fn new() -> Self {
        Self { rules: Vec::new() }
    }

    fn add_rule(&mut self, rule: CssRule) {
        self.rules.push(rule);
    }

    fn rules(&self) -> &Vec<CssRule> {
        &self.rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gosub_shared::traits::css_system::CssDeclaration as _;
    use gosub_shared::traits::css_system::CssRule as _;

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
            CssValue::ColorValue("black".to_string()),
        ]);
        assert_eq!(format!("{}", value), "1px, solid, black");
    }

    #[test]
    fn test_css_declaration() {
        let declaration =
            CssDeclaration::new("color", CssValue::ColorValue("red".to_string()), false);
        assert_eq!(declaration.name(), "color");
        assert_eq!(format!("{}", declaration.value()), "red");
        assert_eq!(declaration.important(), false);
    }

    #[test]
    fn test_css_rule() {
        let mut rule = CssRule::new();
        rule.add_selector("body");
        rule.add_declaration(CssDeclaration::new(
            "color",
            CssValue::ColorValue("red".to_string()),
            false,
        ));
        assert_eq!(rule.selectors(), &vec!["body".to_string()]);
        assert_eq!(rule.declarations().len(), 1);
    }
}
