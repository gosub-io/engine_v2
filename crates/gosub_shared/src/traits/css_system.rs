use crate::traits::document::HasDocument;

pub trait HasCssSystem: Sized {
    type CssStylesheet: CssStylesheet;
    type CssRule: CssRule;
    type CssDeclaration: CssDeclaration;
    type CssValue: CssValue;
}

pub trait HasCssParser: Sized + HasDocument + HasCssSystem {
    type CssParser: CssParser<Self>;
}

pub trait CssParser<C: HasCssSystem>: Sized {
    fn new() -> Self;
    fn parse_str(&mut self, input: &str) -> C::CssStylesheet;
}


pub trait CssValue: Sized {
    fn unit(value: f32, unit: &str) -> Self;
    fn keyword(value: &str) -> Self;
    fn colorvalue(value: &str) -> Self;
    fn list(args: Vec<Self>) -> Self;
    fn is_unit(&self) -> bool;
    fn is_keyword(&self) -> bool;
    fn is_color(&self) -> bool;
    fn is_list(&self) -> bool;
}

pub trait CssDeclaration: Sized + HasCssSystem{
    // type CssValue: CssValue;

    fn new(name: &str, value: Self::CssValue, important: bool) -> Self;
    fn name(&self) -> &str;
    fn value(&self) -> &Self::CssValue;
    fn important(&self) -> bool;
}

pub trait CssRule: Sized + HasCssSystem{
    // type CssDeclaration: CssDeclaration;

    fn new() -> Self;
    fn add_selector(&mut self, selector: &str);
    fn add_declaration(&mut self, declaration: Self::CssDeclaration);
    fn selectors(&self) -> &Vec<String>;
    fn declarations(&self) -> &Vec<Self::CssDeclaration>;
}

pub trait CssStylesheet: Sized + HasCssSystem{
    // type CssRule: CssRule;

    fn new() -> Self;
    fn add_rule(&mut self, rule: Self::CssRule);
    fn rules(&self) -> &Vec<Self::CssRule>;
}
