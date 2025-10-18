use graphina::core::validation::{
    count_components as v_count_components, has_negative_weights as v_has_negative,
    has_self_loops as v_has_self_loops, is_bipartite as v_is_bipartite,
    is_connected as v_is_connected, is_empty as v_is_empty,
};

use crate::PyGraph;

impl PyGraph {
    pub fn is_empty_impl(&self) -> bool {
        v_is_empty(&self.graph)
    }

    pub fn is_connected_impl(&self) -> bool {
        v_is_connected(&self.graph)
    }

    pub fn has_negative_weights_impl(&self) -> bool {
        v_has_negative(&self.graph)
    }

    pub fn has_self_loops_impl(&self) -> bool {
        v_has_self_loops(&self.graph)
    }

    pub fn is_bipartite_impl(&self) -> bool {
        v_is_bipartite(&self.graph)
    }

    pub fn count_components_impl(&self) -> usize {
        v_count_components(&self.graph)
    }
}
