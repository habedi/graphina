use graphina::core::types::NodeId;
use std::collections::HashMap;

/// Helper struct to manage mapping between Python IDs (usize) and Graphina internal NodeIds.
#[derive(Clone, Debug)]
pub struct IdMapper {
    pub(crate) py_to_internal: HashMap<usize, NodeId>,
    pub(crate) internal_to_py: HashMap<NodeId, usize>,
    pub(crate) next_id: usize,
}

impl IdMapper {
    pub fn new() -> Self {
        Self {
            py_to_internal: HashMap::new(),
            internal_to_py: HashMap::new(),
            next_id: 0,
        }
    }

    /// Adds a new mapping for a newly created internal node.
    /// Returns the assigned python ID.
    pub fn add(&mut self, internal_id: NodeId) -> usize {
        let py_id = self.next_id;
        self.py_to_internal.insert(py_id, internal_id);
        self.internal_to_py.insert(internal_id, py_id);
        self.next_id += 1;
        py_id
    }

    /// Adds a mapping with a specific requested Python ID (if available).
    /// Used when preserving IDs during conversion or filtering.
    /// Note: This does NOT automatically check for conflicts, caller must guarantee safety or use with caution.
    /// It updates `next_id` to be max(next_id, py_id + 1).
    pub fn add_with_id(&mut self, internal_id: NodeId, py_id: usize) {
        self.py_to_internal.insert(py_id, internal_id);
        self.internal_to_py.insert(internal_id, py_id);
        if py_id >= self.next_id {
            self.next_id = py_id + 1;
        }
    }

    pub fn remove_by_py_id(&mut self, py_id: usize) -> Option<NodeId> {
        if let Some(nid) = self.py_to_internal.remove(&py_id) {
            self.internal_to_py.remove(&nid);
            Some(nid)
        } else {
            None
        }
    }

    pub fn get_internal(&self, py_id: usize) -> Option<NodeId> {
        self.py_to_internal.get(&py_id).copied()
    }

    pub fn get_py(&self, internal_id: NodeId) -> Option<usize> {
        self.internal_to_py.get(&internal_id).copied()
    }

    pub fn contains_py(&self, py_id: usize) -> bool {
        self.py_to_internal.contains_key(&py_id)
    }

    pub fn clear(&mut self) {
        self.py_to_internal.clear();
        self.internal_to_py.clear();
        self.next_id = 0;
    }
}

impl Default for IdMapper {
    fn default() -> Self {
        Self::new()
    }
}
