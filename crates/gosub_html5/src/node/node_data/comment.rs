#[derive(Debug, Clone)]
pub struct CommentData {
    content: String,
}

impl gosub_shared::traits::node::NodeData for CommentData {}

impl gosub_shared::traits::node::CommentData for CommentData {
    fn new(content: &str) -> Self {
        Self { content: content.into() }
    }

    fn content(&self) -> &str {
        self.content.as_str()
    }
}