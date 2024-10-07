#[derive(Debug, Clone)]
pub struct TextData {
    content: String,
}

impl gosub_shared::traits::node::NodeData for TextData {}

impl gosub_shared::traits::node::TextData for TextData {
    fn new(content: &str) -> Self {
        Self { content: content.into() }
    }

    fn content(&self) -> &str {
        self.content.as_str()
    }
}