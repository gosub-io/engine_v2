use gosub_shared::traits::document::Document;
use gosub_shared::traits::node::{CommentData, DocTypeData, ElementData, TextData, HasNode};
use gosub_shared::traits::node::Node;

pub struct NodeBuilder<N: Node> {
    _marker: std::marker::PhantomData<N>,
}

// impl<N: Node> NodeBuilder<N> {
//     pub fn new_element_node(name: &str, namespace: &str) -> N {
//         N::new(N::NodeData::Element(ElementData::new(name.into(), namespace.into())))
//     }
// 
//     pub fn new_text_node(content: &str) -> N {
//         N::new(N::NodeData::Text(TextData::new(content.into())))
//     }
// 
//     pub fn new_comment_node(content: &str) -> N {
//         N::new(N::NodeData::Comment(CommentData::new(content.into())))
//     }
// 
//     pub fn new_doctype_node(name: &str, public_id: &str, system_id: &str) -> N {
//         N::new(N::NodeData::DocType(DocTypeData::new(name.into(), public_id.into(), system_id.into())))
//     }
// }


impl<N: Node> gosub_shared::traits::node::NodeBuilder<N> for NodeBuilder<N> {
    fn new_element_node(name: &str, namespace: &str) -> N {
        todo!()
    }

    fn new_text_node(content: &str) -> N {
        todo!()
    }

    fn new_comment_node(content: &str) -> N {
        todo!()
    }

    fn new_doctype_node(name: &str, public_id: &str, system_id: &str) -> N {
        todo!()
    }
}