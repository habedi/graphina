//! NetworkX oracle and invariant tests for community detection.
//!
//! Community-detection algorithms are randomized with many near-optimal
//! solutions, so an exact partition match is not meaningful. Instead this test
//! pins the modularity (a stable scalar quality score) of Graphina's Louvain
//! output against NetworkX, and checks the structural invariants every
//! partitioner must satisfy.
//!
//! The corpus (`scripts/gen_oracle_community_fixtures.py`, regenerated with
//! `make oracle-fixtures`) holds planted-partition graphs with clear community
//! structure, the modularity of the ground-truth partition, and the modularity
//! NetworkX's own Louvain achieves. The NetworkX dependency lives only in the
//! generator, so this test stays hermetic.
//!
//! Checks:
//!   - Graphina's Louvain reaches modularity within a slack tolerance of the
//!     NetworkX Louvain reference, computed with the same modularity formula.
//!   - Louvain, Girvan-Newman, label propagation, and Infomap all return a
//!     valid partition/labeling: every node assigned exactly once.

#![cfg(feature = "community")]

use graphina::community::girvan_newman::girvan_newman;
use graphina::community::infomap::infomap;
use graphina::community::label_propagation::label_propagation;
use graphina::community::louvain::louvain;
use graphina::core::types::{Graph, NodeId};
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize)]
struct Corpus {
    cases: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    id: String,
    n: usize,
    blocks: usize,
    edges: Vec<[usize; 2]>,
    #[allow(dead_code)]
    ground_truth_modularity: f64,
    louvain_modularity: f64,
}

/// Slack tolerance on modularity: Louvain is randomized and the two
/// implementations explore different local optima, so this allows Graphina's
/// partition to score modestly below the NetworkX reference while still
/// catching a broken optimizer (which collapses modularity toward 0).
const MOD_SLACK: f64 = 0.07;
const SEED: u64 = 12345;

fn load_corpus() -> Corpus {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/oracle/networkx_community_oracle.json"
    );
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read community oracle corpus at {path}: {e}"));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("failed to parse community oracle: {e}"))
}

fn build_graph(case: &Case) -> (Graph<i32, f64>, Vec<NodeId>) {
    let mut g: Graph<i32, f64> = Graph::new();
    let ids: Vec<NodeId> = (0..case.n).map(|i| g.add_node(i as i32)).collect();
    for e in &case.edges {
        g.add_edge(ids[e[0]], ids[e[1]], 1.0);
    }
    (g, ids)
}

/// Modularity of a node partition on an unweighted undirected graph, using the
/// same definition NetworkX uses: Q = Σ_c (L_c / m − (d_c / 2m)^2), where L_c is
/// the edge count within community c and d_c is the total degree of c. Nodes are
/// identified by index (a `NodeId`'s `index()` equals its insertion order).
fn modularity(n: usize, edges: &[[usize; 2]], communities: &[Vec<usize>]) -> f64 {
    let m = edges.len() as f64;
    if m == 0.0 {
        return 0.0;
    }
    let mut degree = vec![0.0_f64; n];
    for e in edges {
        degree[e[0]] += 1.0;
        degree[e[1]] += 1.0;
    }
    let mut comm_of = vec![usize::MAX; n];
    for (ci, comm) in communities.iter().enumerate() {
        for &node in comm {
            comm_of[node] = ci;
        }
    }
    let mut l_c = vec![0.0_f64; communities.len()];
    for e in edges {
        if comm_of[e[0]] == comm_of[e[1]] {
            l_c[comm_of[e[0]]] += 1.0;
        }
    }
    let mut d_c = vec![0.0_f64; communities.len()];
    for node in 0..n {
        if comm_of[node] != usize::MAX {
            d_c[comm_of[node]] += degree[node];
        }
    }
    let mut q = 0.0;
    for c in 0..communities.len() {
        q += l_c[c] / m - (d_c[c] / (2.0 * m)).powi(2);
    }
    q
}

/// Assert that `partition` assigns every node in 0..n exactly once.
fn assert_valid_partition(n: usize, partition: &[Vec<usize>], who: &str, id: &str) {
    let mut seen = HashSet::new();
    for comm in partition {
        for &node in comm {
            assert!(node < n, "{who}: case {id} node {node} out of range");
            assert!(
                seen.insert(node),
                "{who}: case {id} node {node} assigned twice"
            );
        }
    }
    assert_eq!(
        seen.len(),
        n,
        "{who}: case {id} did not cover all {n} nodes"
    );
}

fn to_index_partition(communities: &[Vec<NodeId>]) -> Vec<Vec<usize>> {
    communities
        .iter()
        .map(|c| c.iter().map(|nid| nid.index()).collect())
        .collect()
}

/// Convert a per-node label vector (label propagation / Infomap output, indexed
/// by node position) into communities of node indices.
fn labels_to_partition(labels: &[usize]) -> Vec<Vec<usize>> {
    use std::collections::HashMap;
    let mut by_label: HashMap<usize, Vec<usize>> = HashMap::new();
    for (node, &label) in labels.iter().enumerate() {
        by_label.entry(label).or_default().push(node);
    }
    by_label.into_values().collect()
}

#[test]
fn oracle_louvain_modularity_matches_networkx() {
    for case in load_corpus().cases {
        let (g, _ids) = build_graph(&case);
        let communities = louvain(&g, Some(SEED))
            .unwrap_or_else(|e| panic!("louvain failed in case {}: {e}", case.id));
        let part = to_index_partition(&communities);
        assert_valid_partition(case.n, &part, "louvain", &case.id);

        let q = modularity(case.n, &case.edges, &part);
        assert!(
            q >= case.louvain_modularity - MOD_SLACK,
            "louvain modularity: case {}: graphina {q} fell more than {MOD_SLACK} below NetworkX {}",
            case.id,
            case.louvain_modularity
        );
    }
}

#[test]
fn invariant_girvan_newman_partitions() {
    for case in load_corpus().cases {
        let (g, _ids) = build_graph(&case);
        let communities = girvan_newman(&g, case.blocks)
            .unwrap_or_else(|e| panic!("girvan_newman failed in case {}: {e}", case.id));
        let part = to_index_partition(&communities);
        assert_valid_partition(case.n, &part, "girvan_newman", &case.id);
    }
}

#[test]
fn invariant_label_propagation_and_infomap_label_every_node() {
    for case in load_corpus().cases {
        let (g, _ids) = build_graph(&case);

        let lp = label_propagation(&g, 100, Some(SEED))
            .unwrap_or_else(|e| panic!("label_propagation failed in case {}: {e}", case.id));
        assert_eq!(
            lp.len(),
            case.n,
            "label_propagation length: case {}",
            case.id
        );
        assert_valid_partition(
            case.n,
            &labels_to_partition(&lp),
            "label_propagation",
            &case.id,
        );

        let im = infomap(&g, 100, Some(SEED))
            .unwrap_or_else(|e| panic!("infomap failed in case {}: {e}", case.id));
        assert_eq!(im.len(), case.n, "infomap length: case {}", case.id);
        assert_valid_partition(case.n, &labels_to_partition(&im), "infomap", &case.id);
    }
}
