use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gosub_css3::css3parser::MyCss3Parser;
use gosub_css3::stylesheet::{CssDeclaration, CssRule, CssStylesheet, CssValue};
use gosub_html5::document::builder::DocumentBuilder;
use gosub_html5::document::document::MyDocument;
use gosub_html5::html5parser::MyHtmlParser;
use gosub_renderer::backend::ratatui::backend::MyRatatuiRenderBackend;
use gosub_renderer::layouter::basic_layouter::BasicLayouter;
use gosub_renderer::render_tree::render_tree::MyRenderTree;
use gosub_renderer::tree_drawer::tree_drawer::MyTreeDrawer;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::css_system::{HasCssParser, HasCssSystem};
use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::html5_parser::{HasHtmlParser, HtmlParser};
use gosub_shared::traits::layouter::HasLayouter;
use gosub_shared::traits::module_conf::ModuleConfiguration;
use gosub_shared::traits::node::ElementData;
use gosub_shared::traits::node::Node;
use gosub_shared::traits::render_backend::HasRenderBackend;
use gosub_shared::traits::render_tree::HasRenderTree;
use gosub_shared::traits::tree_drawer::HasTreeDrawer;

struct MyModuleConfiguration;

impl HasCssSystem for MyModuleConfiguration {
    type CssStylesheet = CssStylesheet;
    type CssRule = CssRule;
    type CssDeclaration = CssDeclaration;
    type CssValue = CssValue;
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

fn bench_test_attach<C: ModuleConfiguration>(c: &mut Criterion) {
    let mut handle = DocumentBuilder::new_document("https://example.com");
    let mut parser = C::HtmlParser::new(handle.clone());
    parser.parse_str("hello world!");

    let node_id = NodeId::new(6);
    let mut binding = handle.get_mut();

    c.bench_function("bench_test_attach", |b| {
        b.iter(|| {
            // Detach node
            if let Some(mut node) = black_box(binding.detach_node(node_id)) {
                // Get the mutable data and add some attributes
                if let Some(data) = node.get_element_data_mut() {
                    data.add_attribute("class", black_box("a b c"));
                    data.add_attribute("id", black_box("myid"));
                    data.add_attribute("foo", black_box("bar"));
                }

                // Finally, reattach the node back into the document/arena
                binding.update_node(node_id, node);
            }
        })
    });
}

fn bench_test_mut<C: ModuleConfiguration>(c: &mut Criterion) {
    let mut handle = DocumentBuilder::new_document("https://example.com");
    let mut parser = C::HtmlParser::new(handle.clone());
    parser.parse_str("hello world!");

    let node_id = NodeId::new(6);
    let mut binding = handle.get_mut();

    c.bench_function("bench_test_mut", |b| {
        b.iter(|| {
            let node = black_box(binding.get_node_mut(node_id).unwrap());

            // Get the mutable data and add some attributes
            if let Some(data) = node.get_element_data_mut() {
                data.add_attribute("class", black_box("a b c"));
                data.add_attribute("id", black_box("myid"));
                data.add_attribute("foo", black_box("bar"));
            }
        })
    });
}

fn bench_test_clone<C: ModuleConfiguration>(c: &mut Criterion) {
    let mut handle = DocumentBuilder::new_document("https://example.com");
    let mut parser = C::HtmlParser::new(handle.clone());
    parser.parse_str("hello world!");

    let node_id = NodeId::new(6);
    let mut binding = handle.get_mut();

    c.bench_function("bench_test_clone", |b| {
        b.iter(|| {
            let mut cloned_node = binding.get_node_clone(node_id).unwrap();

            if let Some(data) = cloned_node.get_element_data_mut() {
                data.add_attribute("class", black_box("a b c"));
                data.add_attribute("id", black_box("myid"));
                data.add_attribute("foo", black_box("bar"));
            }

            binding.update_node(node_id, cloned_node);
        })
    });
}

fn bta(c: &mut Criterion) {
    bench_test_attach::<MyModuleConfiguration>(c);
}

fn btm(c: &mut Criterion) {
    bench_test_mut::<MyModuleConfiguration>(c);
}

fn btc(c: &mut Criterion) {
    bench_test_clone::<MyModuleConfiguration>(c);
}

criterion_group!(benches, bta, btm, btc);
criterion_main!(benches);
