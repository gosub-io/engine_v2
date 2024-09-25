use gosub_css3::MyCssSystem;
use gosub_html5::document::MyDocument;
use gosub_html5::html5parser::MyHtmlParser;
use gosub_renderer::backend::MyRenderBackend;
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
    type Document = MyDocument;
    type HtmlParser = MyHtmlParser;
    type Layouter = MyLayouter;
    type TreeDrawer = MyTreeDrawer;
    type RenderTree = MyRenderTree;
    type RenderBackend = MyRenderBackend;
}

fn main() {
    main_do_things::<MyModuleConfiguration>();
}

fn main_do_things<C: ModuleConfiguration>() {
    let backend = C::RenderBackend::new();
    let css_system = C::CssSystem::new();
    let document = C::Document::new();
    let html_parser = C::HtmlParser::new();
    let layouter = C::Layouter::new();
    let tree_drawer = C::TreeDrawer::new();
    let render_tree = C::RenderTree::new();

    css_system.do_css_things();
    document.do_document_things();
    html_parser.do_html_parser_things();
    layouter.do_layouter_things();
    tree_drawer.do_tree_drawer_things();
    render_tree.do_render_tree_things();
    backend.do_render_backend_things();
}