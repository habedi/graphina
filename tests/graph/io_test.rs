use network::graph::io::{read_edge_list, write_edge_list};
use network::graph::Graph;
use std::fs;
use std::io::Read;

#[test]
fn test_read_edge_list() {
    // Create a temporary edge list file.
    let tmp_path = "tmp_edge_list.txt";
    let edge_list = "1,2,1.5\n2,3,2.0\n3,1,3.0\n";
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
