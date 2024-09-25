use gosub_shared::traits::document::HasDocument;
use gosub_shared::traits::html5_parser::HtmlParser;
use std::marker::PhantomData;

pub struct MyHtmlParser<C: HasDocument> {
    _phantom: PhantomData<C>, // This is won't be needed for the final implementation, since we have the C type parameter anyway
}

impl<C: HasDocument> HtmlParser<C> for MyHtmlParser<C> {
    fn do_html_parser_things(&self) {
        println!("Doing HTML parser things");
    }

    fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

// impl<C: HasDocument> HasCssSystem for MyHtmlParser<C> {
//     type CssSystem = C::CssSystem;
// }
// impl<C: HasDocument> HasDocument<C> for MyHtmlParser<C> {
//     type Document = C::Document;
// }
//
// impl<C: HasDocument> HasHtmlParser<C> for MyHtmlParser<C> {
//     type HtmlParser = Self;
// }
