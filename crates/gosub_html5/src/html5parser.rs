use gosub_shared::document::DocumentHandle;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::document::HasDocument;
use gosub_shared::traits::document::Document;
use gosub_shared::traits::html5_parser::{HtmlParser};
use gosub_shared::traits::node::{NodeBuilder as _};
use crate::node::builder::NodeBuilder;

pub struct MyHtmlParser<C: HasDocument> {
    doc_handle: DocumentHandle<C>,
    parser_state: u32,  // dummy parser state
}

impl<C: HasDocument> HtmlParser<C> for MyHtmlParser<C> {
    fn new(doc_handle: DocumentHandle<C>) -> Self {
        Self {
            doc_handle: doc_handle.clone(),
            parser_state: 0,
        }
    }

    fn parse_str(&mut self, _input: &str) {
        self.parser_state = 1;

        /*
            We generate some dummy nodes here to mimic actual parsing. Generates the following tree:

            <!DOCTYPE html>
               └─ <html>
                  ├─ <head>
                  └─ <body>
                     ├─ <h1>
                     │  └─ This is a header
                     └─ <p>
                        └─ hello world!
         */

        let mut binding = self.doc_handle.get_mut();

        #[allow(type_alias_bounds)]
        type BuilderType<C: HasDocument> = NodeBuilder<C::Node>;

        let node1 = BuilderType::<C>::new_element_node("html", "html");
        let node1_id = binding.register_node_at(node1, NodeId::root(), None);

        let node2 = BuilderType::<C>::new_element_node("head", "html");
        let _node2_id = binding.register_node_at(node2, node1_id, None);

        let node3 = BuilderType::<C>::new_element_node("body", "html");
        let node3_id = binding.register_node_at(node3, node1_id, None);

        let node41 = BuilderType::<C>::new_element_node("h1", "html");
        let node41_id = binding.register_node_at(node41, node3_id, None);

        let node42 = BuilderType::<C>::new_text_node("This is a header");
        let _node42_id = binding.register_node_at(node42, node41_id, None);

        let node4 = BuilderType::<C>::new_element_node("p", "html");
        let node4_id = binding.register_node_at(node4, node3_id, None);

        let node5 = BuilderType::<C>::new_text_node("hello world!");
        let _node5_id = binding.register_node_at(node5, node4_id, None);
    }
}