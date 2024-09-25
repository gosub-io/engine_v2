
pub trait HasCssSystem {
    type CssSystem: CssSystem;
}

pub trait CssSystem: Sized + HasCssSystem<CssSystem = Self> {
    fn do_css_things(&self);

    fn new() -> Self;
}
