use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ElementAttribute {
    name: String,
    value: String,
}

impl gosub_shared::traits::node::ElementAttribute for ElementAttribute {
    fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn value(&self) -> &str {
        self.value.as_str()
    }
}

impl Display for ElementAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}=\"{}\"", self.name, self.value)
    }
}

#[derive(Debug, Clone)]
pub struct ElementData {
    name: String,
    namespace: String,
    attributes: HashMap<String, String>,
    classes: HashMap<String, bool>, // classname -> is active or not
}

impl gosub_shared::traits::node::NodeData for ElementData {}

impl gosub_shared::traits::node::ElementData for ElementData {
    fn new(name: &str, namespace: &str) -> Self {
        Self {
            name: name.into(),
            namespace: namespace.into(),
            attributes: HashMap::new(),
            classes: HashMap::new(),
        }
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn namespace(&self) -> &str {
        self.namespace.as_str()
    }

    fn attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }

    fn add_attribute(&mut self, name: &str, value: &str) {
        self.attributes.insert(name.into(), value.into());

        /// When we add a "class" attribute, we actually store it in the classes map as well.
        if name == "class" {
            for class in value.split_whitespace() {
                self.classes.insert(class.to_string(), true);
            }
        }
    }

    fn remove_attribute(&mut self, name: &str) {
        self.attributes.remove(name);

        if name == "class" {
            self.classes.clear();
        }
    }

    fn classes(&self) -> &HashMap<String, bool> {
        &self.classes
    }

    /// Return all classes for this element node that are marked as active
    fn active_classes(&self) -> Vec<String> {
        self.classes
            .iter()
            .filter(|(_, &active)| active)
            .map(|(name, _)| name.to_string())
            .collect()
    }

    fn add_class(&mut self, name: &str, active: bool) {
        self.classes.insert(name.into(), active);
    }

    fn remove_class(&mut self, name: &str) {
        self.classes.remove(name);
    }

    fn set_class_state(&mut self, name: &str, active: bool) {
        self.classes.insert(name.into(), active);
    }
}
