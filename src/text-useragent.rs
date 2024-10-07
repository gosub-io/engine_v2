use std::thread::sleep;
use gosub_css3::css3parser::MyCss3Parser;
use gosub_css3::MyCssSystem;
use gosub_html5::document::builder::DocumentBuilder;
use gosub_html5::document::document::MyDocument;
use gosub_html5::document::walker::DocumentWalker;
use gosub_html5::html5parser::MyHtmlParser;
use gosub_renderer::backend::ratatui::backend::MyRatatuiRenderBackend;
use gosub_renderer::layouter::basic_layouter::{BasicLayouter, Size};
use gosub_renderer::render_tree::render_tree::MyRenderTree;
use gosub_renderer::tree_drawer::tree_drawer::MyTreeDrawer;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::css_system::{HasCssParser, HasCssSystem};
use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::document::query::{Condition, Query, SearchType};
use gosub_shared::traits::html5_parser::{HasHtmlParser, HtmlParser};
use gosub_shared::traits::layouter::{HasLayouter, Layouter};
use gosub_shared::traits::module_conf::ModuleConfiguration;
use gosub_shared::traits::node::{ElementData, Node};
use gosub_shared::traits::render_backend::{HasRenderBackend, RenderBackend};
use gosub_shared::traits::render_tree::{HasRenderTree, RenderTree};
use gosub_shared::traits::tree_drawer::{HasTreeDrawer};


struct MyModuleConfiguration;

impl HasCssSystem for MyModuleConfiguration {
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
    type RenderBackend = MyRatatuiRenderBackend<Self>;
}

impl HasCssParser for MyModuleConfiguration { type CssParser = MyCss3Parser<Self>; }

impl ModuleConfiguration for MyModuleConfiguration {}

fn main() {
    main_do_things::<MyModuleConfiguration>();
}

fn main_do_things<C: ModuleConfiguration>() {

    let mut handle = DocumentBuilder::new_document("https://example.com");
    let mut html_parser = C::HtmlParser::new(handle.clone());

    html_parser.parse_str("<html><head></head><body><p>Hello world!</p></body></html>");

    println!("-----------------------------------------------");
    let walker = DocumentWalker::new(handle.clone());
    walker.print_tree(handle.clone(), true);

    println!("-----------------------------------------------");
    println!("{:?}", handle.get().get_node_by_element_id("myid"));
    println!("{:?}", handle.get().get_node_by_element_id("foobar"));
    println!("{:?}", handle.get().get_node_by_element_id("myid"));
    println!("{:?}", handle.get().get_node_by_element_id("new-id"));

    let node_id = NodeId::new(6);
    let mut binding = handle.get_mut();
    if let Some(mut node) = binding.detach_node(node_id) {
        if let Some(data) = node.get_element_data_mut() {
            data.add_attribute("class", "foo bar baz");
            data.add_attribute("id", "new-id");
            binding.update_node(node_id, node);
        }
    }
    drop(binding);

    println!("{:?}", handle.get().get_node_by_element_id("myid"));
    println!("{:?}", handle.get().get_node_by_element_id("foobar"));
    println!("{:?}", handle.get().get_node_by_element_id("myid"));
    println!("{:?}", handle.get().get_node_by_element_id("new-id"));

    println!("-----------------------------------------------");
    let walker = DocumentWalker::new(handle.clone());
    walker.print_tree(handle.clone(), true);

    println!("-----------------------------------------------");
    let render_tree = C::RenderTree::from_document(handle.clone());
    let layouter = C::Layouter::from_render_tree(render_tree, Size { width: 800.0, height: 600.0 });

    for box_ in layouter.get_boxes() {
        println!("{}", box_);
    }

    println!("-----------------------------------------------");
    let q = Query::new(
        SearchType::find_all(),
        vec![
            Condition::equals_id("new-id")
        ]
    );
    if let Ok(entries) = handle.get().query(&q) {
        for entry in entries {
            println!("{:?}", entry);
        }
    } else {
        println!("Query failed");
    }

    println!("-----------------------------------------------");
    let mut render_backend = C::RenderBackend::from_layouter(layouter);
    loop {
        render_backend.render_scene();
        sleep(std::time::Duration::from_millis(1000));
    }
}