use gosub_shared::traits::document::HasDocument;
use gosub_shared::traits::html5_parser::{HasHtmlParser, HtmlParser};
use gosub_shared::traits::module_conf::ModuleConfiguration;

struct MyHtmlParser;

// impl<C: ModuleConfiguration> HasHtmlParser<C> for C {
//     // type CssSystem = <Self as ModuleConfiguration>::CssSystem;
//     // type Document = <Self as ModuleConfiguration>::Document;
//     type HtmlParser = <Self as ModuleConfiguration>::HtmlParser;
// }

impl<C: HasDocument> HtmlParser<C> for MyHtmlParser {
    fn do_html_parser_things(&self) {
        println!("Doing HTML parser things");
    }

    fn new() -> Self {
        Self
    }
}
