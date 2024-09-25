use crate::traits::document::HasDocument;

pub trait HasHtmlParser: Sized + HasDocument {
    type HtmlParser: HtmlParser<Self::Document>;
}

pub trait HtmlParser<C: HasDocument>: Sized + HasHtmlParser {
    fn do_html_parser_things(&self);

    fn new() -> Self;
}
