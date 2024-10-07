#[derive(Debug, Clone)]
pub struct DocTypeData {
    name: String,
    public_id: String,
    system_id: String,
}

impl gosub_shared::traits::node::NodeData for DocTypeData {}

impl gosub_shared::traits::node::DocTypeData for DocTypeData {
    fn new(name: &str, public_id: &str, system_id: &str) -> Self {
        Self {
            name: name.into(),
            public_id: public_id.into(),
            system_id: system_id.into(),
        }
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn public_id(&self) -> &str {
        self.public_id.as_str()
    }

    fn system_id(&self) -> &str {
        self.system_id.as_str()
    }
}
