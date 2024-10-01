use gosub_css3::MyCssSystem;
use gosub_html5::document::builder::DocumentBuilder;
use gosub_html5::document::document::MyDocument;
use gosub_html5::document::walker::DocumentWalker;
use gosub_html5::html5parser::MyHtmlParser;
use gosub_renderer::backend::MyRenderBackend;
use gosub_renderer::layouter::MyLayouter;
use gosub_renderer::render_tree::MyRenderTree;
use gosub_renderer::tree_drawer::MyTreeDrawer;
use gosub_shared::traits::css_system::{HasCssSystem};
use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::html5_parser::{HasHtmlParser, HtmlParser};
use gosub_shared::traits::layouter::{HasLayouter};
use gosub_shared::traits::module_conf::ModuleConfiguration;
use gosub_shared::traits::render_backend::{HasRenderBackend};
use gosub_shared::traits::render_tree::{HasRenderTree,};
use gosub_shared::traits::tree_drawer::{HasTreeDrawer};


struct MyModuleConfiguration;

impl HasCssSystem for MyModuleConfiguration {
    type CssSystem = MyCssSystem;
}

impl HasDocument for MyModuleConfiguration {
    type Document = MyDocument<Self>;
    type Node = <Self::Document as Document<Self>>::Node;
}

impl HasHtmlParser for MyModuleConfiguration {
    type HtmlParser = MyHtmlParser<Self>;
}

impl HasLayouter for MyModuleConfiguration {
    type Layouter = MyLayouter<Self>;
}

impl HasRenderTree for MyModuleConfiguration {
    type RenderTree = MyRenderTree<Self>;
}

impl HasTreeDrawer for MyModuleConfiguration {
    type TreeDrawer = MyTreeDrawer<Self>;
}

impl HasRenderBackend for MyModuleConfiguration {
    type RenderBackend = MyRenderBackend;
}

impl ModuleConfiguration for MyModuleConfiguration {}

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