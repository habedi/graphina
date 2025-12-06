use graphina::core::types::{Digraph, Graph, NodeId};
use proptest::prelude::*;
use std::collections::{HashMap, HashSet};

// Define operations that can be performed on the graph
#[derive(Debug, Clone)]
enum GraphOp {
    AddNode(i32),
    AddEdge(i32, i32, f64),
    RemoveNode(i32),
}

// Strategy to generate random graph operations
fn graph_op_strategy() -> impl Strategy<Value = GraphOp> {
    prop_oneof![
        any::<i32>().prop_map(GraphOp::AddNode),
        (any::<i32>(), any::<i32>(), any::<f64>()).prop_map(|(u, v, w)| GraphOp::AddEdge(u, v, w)),
        any::<i32>().prop_map(GraphOp::RemoveNode),
    ]
}

proptest! {
    #[test]
    fn test_graph_invariants(ops in proptest::collection::vec(graph_op_strategy(), 1..50)) {
        let mut graph = Graph::<i32, f64>::new();
        let mut model_nodes: HashSet<i32> = HashSet::new();
        let mut model_to_id: HashMap<i32, NodeId> = HashMap::new();
        let mut edge_count = 0;

        for op in ops {
            match op {
                GraphOp::AddNode(val) => {
                    if !model_nodes.contains(&val) {
                        let id = graph.add_node(val);
                        model_nodes.insert(val);
                        model_to_id.insert(val, id);
                    }
                }
                GraphOp::AddEdge(u, v, w) => {
                    if let (Some(&uid), Some(&vid)) = (model_to_id.get(&u), model_to_id.get(&v)) {
                         graph.add_edge(uid, vid, w);
                         edge_count += 1;
                    }
                }
                GraphOp::RemoveNode(_val) => {}
            }
            prop_assert_eq!(graph.node_count(), model_nodes.len());
            prop_assert_eq!(graph.edge_count(), edge_count);
        }
    }

    #[test]
    fn test_digraph_invariants(ops in proptest::collection::vec(graph_op_strategy(), 1..50)) {
        let mut graph = Digraph::<i32, f64>::new();
        let mut model_nodes: HashSet<i32> = HashSet::new();
        let mut model_to_id: HashMap<i32, NodeId> = HashMap::new();
        let mut edge_count = 0;

        for op in ops {
            match op {
                GraphOp::AddNode(val) => {
                    if !model_nodes.contains(&val) {
                        let id = graph.add_node(val);
                        model_nodes.insert(val);
                        model_to_id.insert(val, id);
                    }
                }
                GraphOp::AddEdge(u, v, w) => {
                    if let (Some(&uid), Some(&vid)) = (model_to_id.get(&u), model_to_id.get(&v)) {
                         graph.add_edge(uid, vid, w);
                         edge_count += 1;
                    }
                }
                GraphOp::RemoveNode(_val) => {}
            }
            prop_assert_eq!(graph.node_count(), model_nodes.len());
            prop_assert_eq!(graph.edge_count(), edge_count);
        }
    }
}
