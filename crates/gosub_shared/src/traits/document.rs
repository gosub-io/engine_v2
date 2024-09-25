use crate::traits::css_system::HasCssSystem;

// pub trait HasDocument<C: HasCssSystem = Self>: Sized + HasCssSystem<C> {
//     type Document: Document<Self>;
// }


pub trait HasDocument: Sized + HasCssSystem {
    type Document: Document<Self::CssSystem>;
}


pub trait Document<C: HasCssSystem>: Sized + HasDocument {
    fn do_document_things(&self);

    fn uses_css_system(&self, css_system: &C::CssSystem);

    fn new() -> Self;
}