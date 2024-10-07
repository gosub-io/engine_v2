use gosub_shared::traits::node::{CommentData, DocTypeData, ElementData, TextData, DocumentData};
use gosub_shared::traits::node::Node;
use gosub_shared::traits::node::NodeBuilder as NodeBuilderTrait;

pub struct NodeBuilder<N: Node> {
    _marker: std::marker::PhantomData<N>,
}

impl<N: Node> NodeBuilderTrait<N> for NodeBuilder<N>
where
    N::NodeData: From<N::ElementData> + From<N::CommentData> + From<N::TextData> + From<N::DocTypeData>
{
    fn new_element_node(name: &str, namespace: &str) -> N {
        let data = N::ElementData::new(name.into(), namespace.into());
        N::new(data.into()).into()
    }

    fn new_text_node(content: &str) -> N {
        let data = N::TextData::new(content.into());
        N::new(data.into())
    }

    fn new_comment_node(content: &str) -> N {
        let data = N::CommentData::new(content.into());
        N::new(data.into())
    }

    fn new_doctype_node(name: &str, public_id: &str, system_id: &str) -> N {
        let data = N::DocTypeData::new(name.into(), public_id.into(), system_id.into());
        N::new(data.into())
    }

    fn new_document_node() -> N {
        let data = N::DocumentData::new();
        N::new(data.into())
    }
}