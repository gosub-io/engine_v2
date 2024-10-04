use gosub_css3::css3parser::MyCss3Parser;
use gosub_css3::MyCssSystem;
use gosub_html5::document::builder::DocumentBuilder;
use gosub_html5::document::document::MyDocument;
use gosub_html5::document::walker::DocumentWalker;
use gosub_html5::html5parser::MyHtmlParser;
use gosub_renderer::backend::text::backend::MyTextRenderBackend;
use gosub_renderer::layouter::basic_layouter::{BasicLayouter, Size};
use gosub_renderer::render_tree::render_tree::MyRenderTree;
use gosub_renderer::tree_drawer::tree_drawer::MyTreeDrawer;
use gosub_shared::traits::css_system::{HasCssParser, HasCssSystem};
use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::html5_parser::{HasHtmlParser, HtmlParser};
use gosub_shared::traits::layouter::{HasLayouter, Layouter};
use gosub_shared::traits::module_conf::ModuleConfiguration;
use gosub_shared::traits::render_backend::{HasRenderBackend};
use gosub_shared::traits::render_tree::{HasRenderTree, RenderTree};
use gosub_shared::traits::tree_drawer::{HasTreeDrawer};


struct MyModuleConfiguration;

impl HasCssSystem for MyModuleConfiguration {
    // type CssSystem = MyCssSystem;
    type CssStylesheet = <MyCssSystem as HasCssSystem>::CssStylesheet;
    type CssRule = <MyCssSystem as HasCssSystem>::CssRule;
    type CssDeclaration = <MyCssSystem as HasCssSystem>::CssDeclaration;
    type CssValue = <MyCssSystem as HasCssSystem>::CssValue;
}

impl HasDocument for MyModuleConfiguration {
    type Document = MyDocument<Self>;
    type Node = <Self::Document as Document<Self>>::Node;
}

impl HasHtmlParser for MyModuleConfiguration {
    type HtmlParser = MyHtmlParser<Self>;
}

impl HasLayouter for MyModuleConfiguration {
    type Layouter = BasicLayouter<Self>;
}

impl HasRenderTree for MyModuleConfiguration {
    type RenderTree = MyRenderTree<Self>;
}

impl HasTreeDrawer for MyModuleConfiguration {
    type TreeDrawer = MyTreeDrawer<Self>;
}

impl HasRenderBackend for MyModuleConfiguration {
    type RenderBackend = MyTextRenderBackend;
}

impl HasCssParser for MyModuleConfiguration { type CssParser = MyCss3Parser<Self>; }

impl ModuleConfiguration for MyModuleConfiguration {}

fn main() {
    main_do_things::<MyModuleConfiguration>();
}

fn main_do_things<C: ModuleConfiguration>() {

    let handle = DocumentBuilder::new_document("https://example.com");
    let mut html_parser = C::HtmlParser::new(handle.clone());

    html_parser.parse_str("<html><head></head><body><p>Hello world!</p></body></html>");

    let walker = DocumentWalker::new(handle.clone());
    walker.print_tree(handle.clone(), true);
    println!("-----------------------------------------------");

    let render_tree = C::RenderTree::from_document(handle.clone());
    let layouter = C::Layouter::from_render_tree(render_tree, Size { width: 800.0, height: 600.0 });

    for box_ in layouter.get_boxes() {
        println!("{}", box_);
    }
    println!("-----------------------------------------------");

    let render_backend = C::RenderBackend::from_layouter(layouter);
    render_backend.render();
}