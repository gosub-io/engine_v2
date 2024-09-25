use gosub_shared::traits::css_system::{CssSystem, HasCssSystem};
use gosub_shared::traits::document::{Document, HasDocument};
use std::marker::PhantomData;

pub struct MyDocument<C: HasCssSystem> {
    _phantom: PhantomData<C>, // This is won't be needed for the final implementation, since we have the C type parameter anyway
}

// impl<C: HasCssSystem> HasDocument<C> for MyDocument<C> {
//     type Document = MyDocument<C>;
// }

// impl<C: HasCssSystem> HasCssSystem for MyDocument<C> {
//     type CssSystem = C::CssSystem;
// }

impl<C: HasCssSystem> Document<C> for MyDocument<C> {
    fn do_document_things(&self) {
        println!("Doing document things");
    }

    fn uses_css_system(&self, css_system: &C::CssSystem) {
        println!("Using css system");

        css_system.do_css_things()
    }

    fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}


impl<C: HasCssSystem> HasCssSystem for MyDocument<C> {
    type CssSystem = C::CssSystem;
}

impl<C: HasCssSystem> HasDocument for MyDocument<C> {
    type Document = MyDocument<C::CssSystem>;
}


// impl<C: HasCssSystem> HasCssSystem for MyDocument<C> {
//     type CssSystem = C::CssSystem;
// }
