use gosub_shared::document::DocumentHandle;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::css_system::HasCssSystem;
use gosub_shared::traits::document::HasDocument;
use gosub_shared::traits::document::Document;
use gosub_shared::traits::html5_parser::{HasHtmlParser, HtmlParser};
use gosub_shared::traits::node::HasNode;
use crate::node::builder::NodeBuilder;

pub struct MyHtmlParser<C: HasDocument + HasNode> {
    doc_handle: DocumentHandle<C>,
    parser_state: u32,  // dummy parser state
}

impl<C: HasDocument> HasHtmlParser for MyHtmlParser<C> { type HtmlParser = MyHtmlParser<C::Document>; }

// impl<C: HasDocument> HasNode for MyHtmlParser<C> {
//     type Node = C::Node;
//     type NodeBuilder = NodeBuilder<Self::Node>;
// }

impl<C: HasDocument> HasNode for MyHtmlParser<C> {
    type Node = C::Node;
    // type NodeBuilder = NodeBuilder<Self::Node>;
}

impl<C: HasDocument> HasDocument for MyHtmlParser<C> { type Document = C::Document; }

impl<C: HasDocument> HasCssSystem for MyHtmlParser<C> { type CssSystem = C::CssSystem; }

impl<C: HasDocument + HasNode> HtmlParser<C> for MyHtmlParser<C> {
    fn new(doc_handle: DocumentHandle<C>) -> Self {
        Self {
            doc_handle: doc_handle.clone(),
            parser_state: 0,
        }
    }

    fn parse_str(&mut self, input: &str) {
        println!("Parsing string: {}", input);
        self.parser_state = 1;

        /*
            We generate some dummy nodes here to mimic actual parsing. Generates the following tree:

            <html>
                <head></head>
                <body>
                    <p>hello world!</p>
                </body>
            </html>
         */

        let mut binding = self.doc_handle.get_mut();
        
        type BuilderType = NodeBuilder<<<C as HasDocument>::Document as HasNode>::Node>;

        let node1 = BuilderType::new_element_node("html", "html");
        let node1_id = binding.register_node_at(node1, NodeId::root(), None);

        let node2 = BuilderType::new_element_node("head", "html");
        let _node2_id = binding.register_node_at(node2, node1_id, None);

        let node3 = BuilderType::new_element_node("body", "html");
        let node3_id = binding.register_node_at(node3, node1_id, None);

        let node4 = BuilderType::new_element_node("p", "html");
        let node4_id = binding.register_node_at(node4, node3_id, None);

        let node5 = BuilderType::new_text_node("hello world!");
        let _node5_id = binding.register_node_at(node5, node4_id, None);
    }
}
