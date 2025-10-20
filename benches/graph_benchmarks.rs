/*!
# Performance Benchmarks for Graphina

This module contains criterion-based benchmarks to measure performance
and detect regressions in graph algorithms.
*/

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use graphina::core::generators::{
    barabasi_albert_graph, complete_graph, erdos_renyi_graph, watts_strogatz_graph,
};
use graphina::core::paths::dijkstra;
use graphina::core::types::Undirected;
use graphina::traversal::{bfs, bidis, dfs};
use std::hint::black_box;

// ============================================================================
// Graph Generator Benchmarks
// ============================================================================

fn bench_erdos_renyi(c: &mut Criterion) {
    let mut group = c.benchmark_group("erdos_renyi_generation");

    for size in [50, 100, 200, 500].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let graph = erdos_renyi_graph::<Undirected>(size, 0.1, 42).unwrap();
                black_box(graph)
            });
        });
    }
    group.finish();
}

fn bench_complete_graph(c: &mut Criterion) {
    let mut group = c.benchmark_group("complete_graph_generation");

    for size in [50, 100, 200, 300].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let graph = complete_graph::<Undirected>(size).unwrap();
                black_box(graph)
            });
        });
    }
    group.finish();
}

fn bench_barabasi_albert(c: &mut Criterion) {
    let mut group = c.benchmark_group("barabasi_albert_generation");

    for size in [50, 100, 200, 500].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let graph = barabasi_albert_graph::<Undirected>(size, 3, 42).unwrap();
                black_box(graph)
            });
        });
    }
    group.finish();
}

fn bench_watts_strogatz(c: &mut Criterion) {
    let mut group = c.benchmark_group("watts_strogatz_generation");

    for size in [50, 100, 200, 500].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let graph = watts_strogatz_graph::<Undirected>(size, 6, 0.3, 42).unwrap();
                black_box(graph)
            });
        });
    }
    group.finish();
}

// ============================================================================
// Graph Traversal Benchmarks
// ============================================================================

fn bench_bfs(c: &mut Criterion) {
    let mut group = c.benchmark_group("bfs_traversal");

    for size in [50, 100, 200, 500, 1000].iter() {
        let graph = erdos_renyi_graph::<Undirected>(*size, 0.1, 42).unwrap();
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();
        let start = nodes[0];

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let result = bfs(&graph, start);
                black_box(result)
            });
        });
    }
    group.finish();
}

fn bench_dfs(c: &mut Criterion) {
    let mut group = c.benchmark_group("dfs_traversal");

    for size in [50, 100, 200, 500, 1000].iter() {
        let graph = erdos_renyi_graph::<Undirected>(*size, 0.1, 42).unwrap();
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();
        let start = nodes[0];

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let result = dfs(&graph, start);
                black_box(result)
            });
        });
    }
    group.finish();
}

fn bench_bidirectional_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("bidirectional_search");

    for size in [50, 100, 200, 500].iter() {
        let graph = erdos_renyi_graph::<Undirected>(*size, 0.1, 42).unwrap();
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();
        let start = nodes[0];
        let target = nodes[nodes.len() - 1];

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let result = bidis(&graph, start, target);
                black_box(result)
            });
        });
    }
    group.finish();
}

// ============================================================================
// Shortest Path Benchmarks
// ============================================================================

fn bench_dijkstra(c: &mut Criterion) {
    let mut group = c.benchmark_group("dijkstra_shortest_path");

    for size in [50, 100, 200, 500].iter() {
        // Use i32 weights for Dijkstra since it requires Ord trait
        let graph = erdos_renyi_graph::<Undirected>(*size, 0.1, 42).unwrap();

        // Convert to graph with i32 weights
        let mut int_graph = graphina::core::types::BaseGraph::<u32, i32, Undirected>::new();
        let node_map: std::collections::HashMap<_, _> = graph
            .nodes()
            .map(|(id, &attr)| {
                let new_id = int_graph.add_node(attr);
                (id, new_id)
            })
            .collect();

        for (src, tgt, _) in graph.edges() {
            int_graph.add_edge(node_map[&src], node_map[&tgt], 1);
        }

        let nodes: Vec<_> = int_graph.nodes().map(|(id, _)| id).collect();
        let start = nodes[0];

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let result = dijkstra(&int_graph, start);
                black_box(result)
            });
        });
    }
    group.finish();
}

// ============================================================================
// Graph Operations Benchmarks
// ============================================================================

fn bench_add_nodes(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_nodes");

    for size in [100, 500, 1000, 5000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut graph = graphina::core::types::Graph::<i32, f32>::new();
                for i in 0..size {
                    graph.add_node(i as i32);
                }
                black_box(graph)
            });
        });
    }
    group.finish();
}

fn bench_add_edges(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_edges");

    for size in [100, 500, 1000, 2000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut graph = graphina::core::types::Graph::<i32, f32>::new();
                let nodes: Vec<_> = (0..size).map(|i| graph.add_node(i as i32)).collect();

                // Add edges in a chain
                for i in 0..size - 1 {
                    graph.add_edge(nodes[i], nodes[i + 1], 1.0);
                }
                black_box(graph)
            });
        });
    }
    group.finish();
}

fn bench_node_removal(c: &mut Criterion) {
    let mut group = c.benchmark_group("node_removal");

    for size in [100, 500, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter_batched(
                || {
                    // Setup: create graph with nodes
                    let mut graph = graphina::core::types::Graph::<i32, f32>::new();
                    let nodes: Vec<_> = (0..size).map(|i| graph.add_node(i as i32)).collect();
                    (graph, nodes)
                },
                |(mut graph, nodes)| {
                    // Remove half the nodes
                    for i in 0..size / 2 {
                        graph.remove_node(nodes[i]);
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
// Graph Density Benchmarks
// ============================================================================

fn bench_sparse_graph_traversal(c: &mut Criterion) {
    let mut group = c.benchmark_group("sparse_graph_bfs");
    group.sample_size(50);

    for size in [100, 500, 1000, 2000].iter() {
        let graph = erdos_renyi_graph::<Undirected>(*size, 0.05, 42).unwrap(); // Sparse: p=0.05
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();
        let start = nodes[0];

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let result = bfs(&graph, start);
                black_box(result)
            });
        });
    }
    group.finish();
}

fn bench_dense_graph_traversal(c: &mut Criterion) {
    let mut group = c.benchmark_group("dense_graph_bfs");
    group.sample_size(30);

    for size in [50, 100, 200, 300].iter() {
        let graph = erdos_renyi_graph::<Undirected>(*size, 0.5, 42).unwrap(); // Dense: p=0.5
        let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();
        let start = nodes[0];

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let result = bfs(&graph, start);
                black_box(result)
            });
        });
    }
    group.finish();
}

// ============================================================================
// Comparison Benchmarks
// ============================================================================

fn bench_traversal_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("traversal_comparison");
    let size = 500;
    let graph = erdos_renyi_graph::<Undirected>(size, 0.1, 42).unwrap();
    let nodes: Vec<_> = graph.nodes().map(|(id, _)| id).collect();
    let start = nodes[0];
    let target = nodes[nodes.len() - 1];

    group.bench_function("bfs", |b| {
        b.iter(|| {
            let result = bfs(&graph, start);
            black_box(result)
        });
    });

    group.bench_function("dfs", |b| {
        b.iter(|| {
            let result = dfs(&graph, start);
            black_box(result)
        });
    });

    group.bench_function("bidirectional", |b| {
        b.iter(|| {
            let result = bidis(&graph, start, target);
            black_box(result)
        });
    });

    group.finish();
}

// ============================================================================
// Benchmark Groups
// ============================================================================

criterion_group!(
    generators,
    bench_erdos_renyi,
    bench_complete_graph,
    bench_barabasi_albert,
    bench_watts_strogatz
);

criterion_group!(traversals, bench_bfs, bench_dfs, bench_bidirectional_search);

criterion_group!(shortest_paths, bench_dijkstra);

criterion_group!(
    operations,
    bench_add_nodes,
    bench_add_edges,
    bench_node_removal
);

criterion_group!(
    density_tests,
    bench_sparse_graph_traversal,
    bench_dense_graph_traversal
);

criterion_group!(comparisons, bench_traversal_comparison);

criterion_main!(
    generators,
    traversals,
    shortest_paths,
    operations,
    density_tests,
    comparisons
);
