use gosub_shared::traits::node::{ElementData, Node};
use std::collections::HashMap;
use gosub_html5::document::tree_iterator::TreeIterator;
use gosub_shared::document::DocumentHandle;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::css_system::{CssDeclaration, HasCssSystem};
use gosub_shared::traits::css_system::{CssRule, CssStylesheet};
use gosub_shared::traits::document::{Document, HasDocument};
use gosub_shared::traits::render_tree::RenderTree;

pub struct MyRenderTree<C: HasDocument + HasCssSystem> {
    /// A map of all properties for each node(id)
    properties: HashMap<NodeId, Vec<C::CssDeclaration>>,
    handle: DocumentHandle<C>,
    marker: std::marker::PhantomData<C>,
}

fn match_selector<C: HasDocument>(handle: DocumentHandle<C>, node_id: NodeId, selector: &str) -> bool {
    // println!("match selector: {:?} {}", node_id, selector);

    let binding = handle.get();
    let Some(node) = binding.get_node(node_id) else {
        // println!("Node is not found");
        return false;
    };

    if let Some(element) = node.get_element_data() {
        // println!("Matching element: {:?}", element.name());
        if element.name() == selector {
            // println!("It's a match!");
            return true;
        }
    } else if let Some(_) = node.get_text_data() {
        // println!("Matching text: {:?}", data.content());
        return false;
    // } else {
        // println!("Node is not an element or text");
    }

    // println!("Nothing matched");
    false
}

impl<C: HasDocument> RenderTree<C> for MyRenderTree<C> {
    fn from_document(handle: DocumentHandle<C>) -> Self {
        let mut node_properties = HashMap::new();

        // Iterate all nodes in the document
        for node_id in TreeIterator::new(handle.clone()) {

            // Skip any nodes immediately that are not renderable (comments, doctypes etc)
            if let Some(node) = handle.get().get_node(node_id) {
                if ! node.is_renderable() {
                    continue;
                }
            }

            // Iterate each stylesheet
            for stylesheet in handle.get().stylesheets() {
                for rule in stylesheet.rules() {
                    for selector in rule.selectors() {
                        if ! match_selector(handle.clone(), node_id, selector) {
                            continue;
                        }

                        let mut decls = vec![];
                        // Selector matches, so we need to add the declarations to the node
                        for declaration in rule.declarations() {
                            decls.push(C::CssDeclaration::new(
                                declaration.name(),
                                declaration.value(),
                                declaration.important()
                            ));
                        }

                        node_properties.insert(node_id, decls);
                    }
                }
            }
        }

        Self {
            properties: node_properties,
            handle: handle,
            marker: Default::default(),
        }
    }

    fn get_property(&self, node_id: NodeId, prop_name: &str) -> Option<&Vec<C::CssDeclaration>> {
        if let Some(props) = self.properties.get(&node_id) {
            // @TODO: This can be optimized by using a hashmap
            for prop in props {
                if prop.name() == prop_name {
                    return Some(props);
                }
            }
        }

        None
    }

    fn get_properties(&self, node_id: NodeId) -> Option<&Vec<C::CssDeclaration>> {
        self.properties.get(&node_id)
    }

    fn get_handle(&self) -> DocumentHandle<C> {
        self.handle.clone()
    }
}