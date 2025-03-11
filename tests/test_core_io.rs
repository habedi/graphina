use graphina::core::io::{
    read_adjacency_list, read_edge_list, write_adjacency_list, write_edge_list,
};
use graphina::core::types::Graph;
use std::collections::HashMap;
use std::fs;
use std::io::Read;

#[test]
fn test_read_edge_list() {
    // Create a temporary edge list file that includes comments.
    let tmp_path = "tmp_edge_list.txt";
    let edge_list = "\
# This is a comment line and should be ignored
1,2,1.5
2,3,2.0
3,1,3.0  # Comment after data should be ignored
";
    fs::write(tmp_path, edge_list).expect("Unable to write temporary file");

    // Create an undirected graph and read the edge list.
    let mut graph = Graph::<i32, f32>::new();
    read_edge_list(tmp_path, &mut graph, ',').expect("read_edge_list failed");

    // Verify that we have 3 nodes and 3 edges.
    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 3);

    // Clean up.
    fs::remove_file(tmp_path).expect("Failed to remove temporary file");
}

#[test]
fn test_write_edge_list() {
    // Create an undirected graph.
    let mut graph = Graph::<i32, f32>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    graph.add_edge(n1, n2, 1.5);
    graph.add_edge(n2, n3, 2.0);
    graph.add_edge(n3, n1, 3.0);

    // Write the graph to a temporary file.
    let tmp_path = "tmp_edge_list_out.txt";
    write_edge_list(tmp_path, &graph, ',').expect("write_edge_list failed");

    // Read the file contents.
    let mut content = String::new();
    fs::File::open(tmp_path)
        .expect("Failed to open output file")
        .read_to_string(&mut content)
        .expect("Failed to read output file");

    // Check that each expected edge appears in the file (order may vary).
    assert!(content.contains("1,2,1.5") || content.contains("2,1,1.5"));
    assert!(content.contains("2,3,2") || content.contains("3,2,2"));
    assert!(content.contains("3,1,3") || content.contains("1,3,3"));

    // Clean up.
    fs::remove_file(tmp_path).expect("Failed to remove temporary file");
}

#[test]
fn test_read_adjacency_list() {
    // Create a temporary adjacency list file with comments.
    let tmp_path = "tmp_adj_list.txt";
    let adj_list = "\
# Adjacency list with comments
1,2,1.5,3,2.5  # Node 1 has neighbors 2 and 3
2,3,2.0
3
";
    fs::write(tmp_path, adj_list).expect("Unable to write temporary file");

    // Create an undirected graph and read the adjacency list.
    let mut graph = Graph::<i32, f32>::new();
    read_adjacency_list(tmp_path, &mut graph, ',').expect("read_adjacency_list failed");

    // Expected:
    // Line "1,2,1.5,3,2.5" adds edges: (1,2) with 1.5 and (1,3) with 2.5.
    // Line "2,3,2.0" adds edge: (2,3) with 2.0.
    // Total: 3 nodes and 3 edges.
    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 3);

    // Clean up.
    fs::remove_file(tmp_path).expect("Failed to remove temporary file");
}

#[test]
fn test_write_adjacency_list() {
    // Create an undirected graph.
    let mut graph = Graph::<i32, f32>::new();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    graph.add_edge(n1, n2, 1.5);
    graph.add_edge(n1, n3, 2.5);
    graph.add_edge(n2, n3, 2.0);

    // Write the adjacency list to a temporary file.
    let tmp_path = "tmp_adj_list_out.txt";
    write_adjacency_list(tmp_path, &graph, ',').expect("write_adjacency_list failed");

    // Read the file contents.
    let mut content = String::new();
    fs::File::open(tmp_path)
        .expect("Failed to open output file")
        .read_to_string(&mut content)
        .expect("Failed to read output file");

    // Parse the file content into a map: node -> Vec<(neighbor, weight)>
    let mut output: HashMap<i32, Vec<(i32, f32)>> = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Split on the separator
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        // First token is the node value.
        let node: i32 = parts[0].parse().unwrap();
        let mut nbrs = Vec::new();
        // Each remaining token should be in the format "nbr:weight"
        for token in parts.iter().skip(1) {
            if token.is_empty() {
                continue;
            }
            let subparts: Vec<&str> = token.split(':').collect();
            assert_eq!(subparts.len(), 2, "Token format should be nbr:weight");
            let nbr: i32 = subparts[0].parse().unwrap();
            let weight: f32 = subparts[1].parse().unwrap();
            nbrs.push((nbr, weight));
        }
        // Sorting the neighbor list to ensure order doesn't affect our comparison.
        nbrs.sort_by_key(|&(nbr, _)| nbr);
        output.insert(node, nbrs);
    }

    // Expected mapping:
    // Node 1 should have neighbors 2 (1.5) and 3 (2.5)
    // Node 2 should have neighbor 3 (2.0)
    // Node 3 should have no neighbors.
    let mut expected: HashMap<i32, Vec<(i32, f32)>> = HashMap::new();
    expected.insert(1, vec![(2, 1.5), (3, 2.5)]);
    expected.insert(2, vec![(3, 2.0)]);
    expected.insert(3, Vec::new());

    assert_eq!(output, expected);

    // Clean up.
    fs::remove_file(tmp_path).expect("Failed to remove temporary file");
}
