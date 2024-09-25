use gosub_shared::traits::html5_parser::HasHtmlParser;
use gosub_shared::traits::module_conf::ModuleConfiguration;

impl<C: ModuleConfiguration> HasHtmlParser<C> for C {
    // type CssSystem = <Self as ModuleConfiguration>::CssSystem;
    // type Document = <Self as ModuleConfiguration>::Document;
    type HtmlParser = <Self as ModuleConfiguration>::HtmlParser;
}