#[derive(Debug, Clone)]
pub struct DocumentData;

impl gosub_shared::traits::node::NodeData for DocumentData {}

impl gosub_shared::traits::node::DocumentData for DocumentData {
    fn new() -> Self {
        Self
    }
}
