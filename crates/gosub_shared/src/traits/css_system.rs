pub trait HasCssSystem: Sized {
    type CssSystem: CssParser;
}

pub trait CssValue: Sized {}

pub trait CssDeclaration: Sized {
    type CssValue: CssValue;

    fn new(name: &str, value: Self::CssValue, important: bool) -> Self;
    fn name(&self) -> &str;
    fn value(&self) -> &Self::CssValue;
    fn important(&self) -> bool;
}

pub trait CssRule: Sized {
    type CssDeclaration: CssDeclaration;

    fn new() -> Self;
    fn add_selector(&mut self, selector: &str);
    fn add_declaration(&mut self, declaration: Self::CssDeclaration);
    fn selectors(&self) -> &Vec<String>;
    fn declarations(&self) -> &Vec<Self::CssDeclaration>;
}

pub trait CssStylesheet: Sized {
    type CssRule: CssRule;

    fn new() -> Self;
    fn add_rule(&mut self, rule: Self::CssRule);
    fn rules(&self) -> &Vec<Self::CssRule>;
}

pub trait CssParser: Sized {
    type CssStylesheet: CssStylesheet;

    fn parse(input: &str) -> Self::CssStylesheet;
}
