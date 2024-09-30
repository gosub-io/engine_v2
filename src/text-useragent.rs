use gosub_css3::MyCssSystem;
use gosub_html5::document::builder::DocumentBuilder;
use gosub_html5::document::document::MyDocument;
use gosub_html5::document::walker::DocumentWalker;
use gosub_html5::html5parser::MyHtmlParser;
use gosub_renderer::text_backend::MyTextRenderBackend;
use gosub_renderer::layouter::MyLayouter;
use gosub_renderer::render_tree::MyRenderTree;
use gosub_renderer::tree_drawer::MyTreeDrawer;
use gosub_shared::traits::css_system::CssSystem;
use gosub_shared::traits::document::Document;
use gosub_shared::traits::html5_parser::HtmlParser;
use gosub_shared::traits::layouter::Layouter;
use gosub_shared::traits::module_conf::ModuleConfiguration;
use gosub_shared::traits::render_backend::RenderBackend;
use gosub_shared::traits::render_tree::RenderTree;
use gosub_shared::traits::tree_drawer::TreeDrawer;

struct MyModuleConfiguration;

impl ModuleConfiguration for MyModuleConfiguration {
    type CssSystem = MyCssSystem;
    type Document = MyDocument<Self>;
    type HtmlParser = MyHtmlParser<Self>;
    type Layouter = MyLayouter;
    type TreeDrawer = MyTreeDrawer;
    type RenderTree = MyRenderTree;
    type RenderBackend = MyTextRenderBackend;
}

fn main() {
    main_do_things::<MyModuleConfiguration>();
}

fn main_do_things<C: ModuleConfiguration>() {

    let handle = DocumentBuilder::new_document("https://example.com");
    let mut html_parser = C::HtmlParser::new(handle.clone());

    html_parser.parse_str("<html><head></head><body><p>Hello world!</p></body></html>");

    let walker = DocumentWalker::new(handle.clone());
    walker.print_tree(handle.clone());
}