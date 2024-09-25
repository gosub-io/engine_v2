pub trait HasCssSystem {
    type CssSystem: CssSystem;
}

pub trait CssSystem: Sized {
    fn do_css_things(&self);

    fn new() -> Self;
}
