use crate::traits::document::HasDocument;

pub trait HasHtmlParser<D: HasDocument>: Sized {
    // type CssSystem: CssSystem;
    // type Document: Document<Self>;
    type HtmlParser: HtmlParser<D>;
}

// impl<HP: HasHtmlParser> HasDocument for HP {
//     type CssSystem = HP::CssSystem;
//     type Document = HP::Document;
// }


pub trait HtmlParser<C: HasDocument>: Sized {
    fn do_html_parser_things(&self);

    fn new() -> Self;
}

// impl<P: HtmlParser<D>, D: Document<C>, C: CssSystem> HasHtmlParser for P {
//     type CssSystem = C;
//     type Document = D;
//     type HtmlParser = P;
// }