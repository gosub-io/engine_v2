use crate::traits::css_system::HasCssSystem;

pub trait HasDocument: Sized + HasCssSystem {
    type Document: Document<Self>;
}

pub trait Document<C: HasCssSystem>: Sized {
    fn do_document_things(&self);

    fn uses_css_system(&self, css_system: &C::CssSystem);

    fn new() -> Self;
}
