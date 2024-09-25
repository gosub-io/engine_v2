use gosub_shared::traits::document::HasDocument;
use gosub_shared::traits::html5_parser::HtmlParser;

pub struct MyHtmlParser;

impl<C: HasDocument> HtmlParser<C> for MyHtmlParser {
    fn do_html_parser_things(&self) {
        println!("Doing HTML parser things");
    }

    fn new() -> Self {
        Self
    }
}
