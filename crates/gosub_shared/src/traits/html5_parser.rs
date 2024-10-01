use crate::document::DocumentHandle;
use crate::traits::document::HasDocument;

pub trait HasHtmlParser: Sized + HasDocument {
    type HtmlParser: HtmlParser<Self>;
}

// impl<HP: HasHtmlParser> HasDocument for HP {
//     type CssSystem = HP::CssSystem;
//     type Document = HP::Document;
// }

pub trait HtmlParser<C: HasDocument>: Sized {
    fn new(doc_handle: DocumentHandle<C>) -> Self;
    fn parse_str(&mut self, input: &str);
}

// impl<P: HtmlParser<D>, D: Document<C>, C: CssSystem> HasHtmlParser for P {
//     type CssSystem = C;
//     type Document = D;
//     type HtmlParser = P;
// }