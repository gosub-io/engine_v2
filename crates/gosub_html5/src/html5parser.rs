use gosub_shared::document::DocumentHandle;
use gosub_shared::location::Location;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::css_system::{CssParser, HasCssParser};
use gosub_shared::traits::document::Document;
use gosub_shared::traits::document::HasDocument;
use gosub_shared::traits::html5_parser::HtmlParser;
use gosub_shared::document::task_queue::{DocumentTask, DocumentTaskQueue};

/// The HTML parser implementation will parse an input string (or stream reader) and generate a
/// DOM tree based on the input. Instead of generating the tree directly, it will use the Document
/// Task Queue to store tasks that will be executed during a flush of the queue. This will basically
/// defer the actual creation until a later time.

pub struct MyHtmlParser<C: HasDocument + HasCssParser> {
    doc_handle: DocumentHandle<C>,
    parser_state: u32, // dummy parser state
}

impl<C: HasDocument + HasCssParser> HtmlParser<C> for MyHtmlParser<C> {
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

        let mut task_queue = DocumentTaskQueue::new(self.doc_handle.clone());

        let node_id_1 = task_queue.add_task(DocumentTask::CreateElement {
            name: "html".to_string(),
            namespace: "html".to_string(),
            location: Location::default(),
            parent_id: NodeId::root(),
            position: None
        });

        let _ = task_queue.add_task(DocumentTask::CreateElement {
            name: "head".to_string(),
            namespace: "html".to_string(),
            location: Default::default(),
            parent_id: node_id_1,
            position: None
        });

        let node_id_3 = task_queue.add_task(DocumentTask::CreateElement {
            name: "body".to_string(),
            namespace: "html".to_string(),
            location: Default::default(),
            parent_id: node_id_1,
            position: None
        });

        let node_id_4_1 = task_queue.add_task(DocumentTask::CreateElement {
            name: "h1".to_string(),
            namespace: "html".to_string(),
            location: Default::default(),
            parent_id: node_id_3,
            position: None
        });

        let _ = task_queue.add_task(DocumentTask::CreateText {
            content: "This is a header".to_string(),
            location: Default::default(),
            parent_id: node_id_4_1,
            position: None
        });

        let node_id_4 = task_queue.add_task(DocumentTask::CreateElement {
            name: "p".to_string(),
            namespace: "html".to_string(),
            location: Default::default(),
            parent_id: node_id_3,
            position: None
        });

        let _ = task_queue.add_task(DocumentTask::CreateText {
            content: "hello world!".to_string(),
            location: Default::default(),
            parent_id: node_id_4,
            position: None
        });

        let _ = task_queue.add_task(DocumentTask::CreateText {
            content: "prefix".to_string(),
            location: Default::default(),
            parent_id: node_id_1,
            position: Some(0)
        });

        let _ = task_queue.add_task(DocumentTask::InsertAttribute {
            key: "class".to_string(),
            value: "a b c".to_string(),
            location: Default::default(),
            node_id: node_id_3,
        });

        let _ = task_queue.add_task(DocumentTask::InsertAttribute {
            key: "id".to_string(),
            value: "myid".to_string(),
            location: Default::default(),
            node_id: node_id_3,
        });

        let _ = task_queue.add_task(DocumentTask::InsertAttribute {
            key: "foo".to_string(),
            value: "bar".to_string(),
            location: Default::default(),
            node_id: node_id_3,
        });


        let errors = task_queue.flush();
        if !errors.is_empty() {
            for error in errors {
                println!("Parse Error: {}", error);
            }
        }
 */

        // We also mimic some CSS style parsing here.
        let mut parser = C::CssParser::new();
        let stylesheet = parser.parse_str("h1 { color: red; }");
        let mut binding = self.doc_handle.get_mut();
        binding.add_stylesheet(stylesheet);
    }
}
