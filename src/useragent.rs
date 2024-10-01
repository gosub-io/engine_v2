use gosub_css3::MyCssSystem;
use gosub_html5::document::document::MyDocument;
use gosub_html5::html5parser::MyHtmlParser;
use gosub_renderer::backend::MyRenderBackend;
use gosub_renderer::layouter::MyLayouter;
use gosub_renderer::render_tree::MyRenderTree;
use gosub_renderer::tree_drawer::MyTreeDrawer;
use gosub_shared::document::DocumentHandle;
use gosub_shared::traits::css_system::{CssSystem, HasCssSystem};
use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::html5_parser::{HasHtmlParser, HtmlParser};
use gosub_shared::traits::layouter::{HasLayouter, Layouter};
use gosub_shared::traits::module_conf::ModuleConfiguration;
use gosub_shared::traits::render_backend::{HasRenderBackend, RenderBackend};
use gosub_shared::traits::render_tree::{HasRenderTree, RenderTree};
use gosub_shared::traits::tree_drawer::{HasTreeDrawer, TreeDrawer};

struct MyModuleConfiguration;

impl HasCssSystem for MyModuleConfiguration {
    type CssSystem = MyCssSystem;
}

impl HasDocument for MyModuleConfiguration {
    type Document = MyDocument<Self::CssSystem>;
}

impl HasHtmlParser for MyModuleConfiguration {
    type HtmlParser = MyHtmlParser<Self::Document>;
}

impl HasLayouter for MyModuleConfiguration {
    type Layouter = MyLayouter<Self::RenderTree>;
}

impl HasRenderTree for MyModuleConfiguration {
    type RenderTree = MyRenderTree<Self::Document>;
}

impl HasTreeDrawer for MyModuleConfiguration {
    type TreeDrawer = MyTreeDrawer<Self::Layouter>;
}

impl HasRenderBackend for MyModuleConfiguration {
    type RenderBackend = MyRenderBackend;
}

impl ModuleConfiguration for MyModuleConfiguration {}


fn main() {
    main_do_things::<MyModuleConfiguration>();
}

fn main_do_things<C: ModuleConfiguration>() {
    let backend = C::RenderBackend::new();
    let css_system = C::CssSystem::new();
    let document =  DocumentHandle::<C>::new(C::Document::new("https://example.com"));
    let html_parser = C::HtmlParser::new(document.clone());
    let layouter = C::Layouter::new();
    let tree_drawer = C::TreeDrawer::new();
    let render_tree = C::RenderTree::new();
    
    
    

    css_system.do_css_things();
    document.do_document_things();
    html_parser.do_html_parser_things();
    layouter.do_layouter_things();
    tree_drawer.do_tree_drawer_things();
    render_tree.do_render_tree_things(&document);
    backend.do_render_backend_things();
}