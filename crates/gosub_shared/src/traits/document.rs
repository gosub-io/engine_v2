use crate::traits::css_system::{CssSystem, HasCssSystem};

pub trait HasDocument: Sized {
    type CssSystem: CssSystem;
    type Document: Document<Self>;
}

impl<HD: HasDocument> HasCssSystem for HD {
    type CssSystem = HD::CssSystem;
}

pub trait Document<C: HasCssSystem>: Sized {
    fn do_document_things(&self);

    fn new() -> Self;
}
