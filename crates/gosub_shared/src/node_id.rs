use std::fmt;
use std::fmt::{Display, Formatter};

/// A node ID is an identifier for nodes found inside a Document. Note that by convention, node ID 0
/// references the root of a document / tree structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId {
    id: u32,
}

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "NodeId({})", self.id)
    }
}

impl NodeId {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn root() -> Self {
        Self { id: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodeid() {
        let id = NodeId::new(42);
        assert_eq!(id.id(), 42);
    }

    #[test]
    fn test_root() {
        let id = NodeId::root();
        assert_eq!(id.id(), 0);
    }

    #[test]
    fn test_clone() {
        let id = NodeId::new(42);
        let id2 = id.clone();
        assert_eq!(id.id(), id2.id());

        // Has copy, so it can be copied without clone
        let id3 = id2;
        assert_eq!(id3, id2);
    }

    #[test]
    fn test_display() {
        let id = NodeId::new(42);
        assert_eq!(format!("{}", id), "NodeId(42)");
    }
}
