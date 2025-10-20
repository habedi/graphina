/*!
# Integration Benchmarks

End-to-end performance benchmarks for complete workflows and cross-module operations.
These benchmarks test realistic usage patterns and ensure module interactions are efficient.
*/

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use graphina::core::generators::{barabasi_albert_graph, erdos_renyi_graph};
use graphina::core::types::{Graph, Undirected};
use std::hint::black_box;

#[cfg(feature = "centrality")]
use graphina::centrality::pagerank::pagerank;

#[cfg(feature = "community")]
use graphina::community::louvain::louvain;

#[cfg(feature = "mst")]
use graphina::mst::kruskal_mst;

use ordered_float::OrderedFloat;

// ============================================================================
// Complete Data Analysis Pipeline
// ============================================================================

#[cfg(all(feature = "centrality", feature = "community"))]
fn bench_complete_analysis_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("complete_analysis_pipeline");

    for size in [100, 200, 500].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                // Generate graph
                let graph = erdos_renyi_graph::<Undirected>(size, 0.1, 42).unwrap();

                // Convert for algorithms
                let mut graph_f64 = Graph::<u32, f64>::new();
                let node_map: std::collections::HashMap<_, _> = graph
                    .nodes()
                    .map(|(nid, &attr)| (nid, graph_f64.add_node(attr)))
                    .collect();

                for (src, tgt, &weight) in graph.edges() {
                    graph_f64.add_edge(node_map[&src], node_map[&tgt], weight as f64);
                }

                // Run community detection
                let communities = louvain(&graph_f64, Some(42));

                // Run centrality analysis
                let pr = pagerank(&graph_f64, 0.85, 50, 1e-4).unwrap();

                black_box((communities, pr))
            });
        });
    }

    group.finish();
}

// ============================================================================
// Graph Construction and Analysis
// ============================================================================

fn bench_build_and_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_and_query");

    for size in [100, 500, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                // Build graph
                let mut graph = Graph::<i32, f32>::new();
                let nodes: Vec<_> = (0..size).map(|i| graph.add_node(i as i32)).collect();

                // Add edges
                for i in 0..size {
                    for j in (i + 1)..size.min(i + 10) {
                        graph.add_edge(nodes[i], nodes[j], 1.0);
                    }
                }

                // Query operations
                let mut total_degree = 0;
                for node in &nodes[..nodes.len().min(50)] {
                    total_degree += graph.degree(*node).unwrap_or(0);
                }

                black_box((graph, total_degree))
            });
        });
    }

    group.finish();
}

// ============================================================================
// Graph Transformation Pipeline
// ============================================================================

#[cfg(feature = "mst")]
fn bench_graph_transformation(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_transformation");

    for size in [50, 100, 200].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                // Generate base graph
                let graph = erdos_renyi_graph::<Undirected>(size, 0.2, 42).unwrap();

                // Convert to OrderedFloat for MST
                let mut mst_graph = Graph::<u32, OrderedFloat<f64>>::new();
                let node_map: std::collections::HashMap<_, _> = graph
                    .nodes()
                    .map(|(nid, &attr)| (nid, mst_graph.add_node(attr)))
                    .collect();

                for (src, tgt, &weight) in graph.edges() {
                    mst_graph.add_edge(node_map[&src], node_map[&tgt], OrderedFloat(weight as f64));
                }

                // Compute MST
                let mst_result = kruskal_mst(&mst_graph);

                black_box(mst_result)
            });
        });
    }

    group.finish();
}

// ============================================================================
// Incremental Updates
// ============================================================================

fn bench_incremental_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("incremental_updates");

    for size in [100, 500, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter_batched(
                || {
                    // Setup: create initial graph
                    barabasi_albert_graph::<Undirected>(size, 3, 42).unwrap()
                },
                |mut graph| {
                    // Simulate incremental updates
                    let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();

                    // Add new nodes and connect them
                    for i in 0..10 {
                        let new_node = graph.add_node((size + i) as u32);
                        // Connect to random existing nodes
                        for j in 0..3.min(nodes.len()) {
                            graph.add_edge(new_node, nodes[j], (i + j) as f32);
                        }
                    }

                    black_box(graph)
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

// ============================================================================
// Memory and Clone Performance
// ============================================================================

fn bench_clone_and_copy(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone_performance");

    for size in [100, 500, 1000].iter() {
        let graph = barabasi_albert_graph::<Undirected>(*size, 3, 42).unwrap();

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let cloned = graph.clone();
                black_box(cloned)
            });
        });
    }

    group.finish();
}

// ============================================================================
// Benchmark Groups
// ============================================================================

#[cfg(all(feature = "centrality", feature = "community", feature = "mst"))]
criterion_group!(
    benches,
    bench_complete_analysis_pipeline,
    bench_build_and_query,
    bench_graph_transformation,
    bench_incremental_updates,
    bench_clone_and_copy,
);

#[cfg(all(feature = "centrality", feature = "community", not(feature = "mst")))]
criterion_group!(
    benches,
    bench_complete_analysis_pipeline,
    bench_build_and_query,
    bench_incremental_updates,
    bench_clone_and_copy,
);

#[cfg(not(all(feature = "centrality", feature = "community")))]
criterion_group!(
    benches,
    bench_build_and_query,
    bench_incremental_updates,
    bench_clone_and_copy,
);

criterion_main!(benches);
