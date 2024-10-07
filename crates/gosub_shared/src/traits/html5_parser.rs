use crate::document::DocumentHandle;
use crate::traits::document::HasDocument;

pub trait HasHtmlParser: Sized + HasDocument {
    type HtmlParser: HtmlParser<Self>;
}

pub trait HtmlParser<C: HasDocument>: Sized {
    fn new(doc_handle: DocumentHandle<C>) -> Self;
    fn parse_str(&mut self, input: &str);
}