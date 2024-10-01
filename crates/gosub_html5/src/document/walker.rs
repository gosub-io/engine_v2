use gosub_shared::document::DocumentHandle;
use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::node::{CommentData};
use gosub_shared::traits::node::DocTypeData;
use gosub_shared::traits::node::ElementData;
use gosub_shared::traits::node::TextData;
use gosub_shared::traits::node::Node;
use std::io::Write;

pub struct DocumentWalker<C: HasDocument> {
    pub doc_handle: DocumentHandle<C>,
}

impl<C: HasDocument> DocumentWalker<C> {
    pub fn new(doc_handle: DocumentHandle<C>) -> Self {
        Self {
            doc_handle: doc_handle.clone(),
        }
    }

    pub fn print_tree(&self, doc_handle: DocumentHandle<C>) {
        let document = doc_handle.get();
        if let Some(root_node) = document.get_root_node() {
            self.print_element(root_node, "".to_string(), true, &mut std::io::stdout());
        }
    }

    fn print_element(&self, node: &C::Node, prefix: String, last: bool, f: &mut impl Write) {
        let mut buffer = prefix.clone();
        if last {
            buffer.push_str("└─ ");
        } else {
            buffer.push_str("├─ ");
        }

        if let Some(data) = node.get_doctype_data() {
            let pid = if data.public_id().is_empty() { "" } else { &format!(" PUBLIC \"{}\"", data.public_id()).to_string() };
            let sid = if data.system_id().is_empty() { "" } else { &format!("\"{}\"", data.system_id()).to_string() };
            writeln!(f, "{}<!DOCTYPE {}{}{}>", buffer, data.name(), pid, sid).unwrap();
        }
        if let Some(data) = node.get_text_data() {
            writeln!(f, "{}{}", buffer, data.content()).unwrap();
        }
        if let Some(data) = node.get_comment_data() {
            writeln!(f, "{}<!-- {} -->", buffer, data.content()).unwrap();
        }
        if let Some(data) = node.get_element_data() {
            writeln!(f, "{}<{}>", buffer, data.name()).unwrap();
            for (i, attr) in data.attributes().iter().enumerate() {
                let last = i == data.attributes().len() - 1;
                writeln!(f, "{}    {}{}", buffer, attr, if last { "" } else { "," }).unwrap();
            }
            writeln!(f, "{}Element", buffer).unwrap();
        }

        for (i, child) in node.children().iter().enumerate() {
            let binding = self.doc_handle.get();
            let child = binding.get_node(*child);

            let last = i == node.children().len() - 1;
            self.print_element(child.unwrap(), format!("{}{}", prefix, if last { "    " } else { "│   " }), last, f);
        }
    }
}
