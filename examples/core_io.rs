use graphina::core::io::{read_edge_list, write_adjacency_list, write_edge_list};
use graphina::core::types::Graph;

// Use `make testdata` to download the test datasets if needed.

const TESTDATA_DIR: &str = "tests/testdata/graphina-graphs";

fn main() {
    // Parameters
    let path = format!("{}/dblp_citation_network.txt", TESTDATA_DIR);
    let mut graph = Graph::<i32, f32>::new();
    let sep = '\t';

    // Read the edge list from the file
    let _ = read_edge_list_dataset(&path, &mut graph, sep);

    // Write the adjacency list to a file
    let path_for_adjacency_list =
        format!("{}/dblp_citation_network_adjacency_list.txt", TESTDATA_DIR);
    let _ = write_adjacency_list_dataset(&path_for_adjacency_list, &graph, sep);

    // Write the edge list to a file
    let path_for_edge_list = format!("{}/dblp_citation_network_edge_list.txt", TESTDATA_DIR);
    let _ = write_edge_list_dataset(&path_for_edge_list, &graph, sep);

    // Have a look at the graph
    println!("==========================================================");
    let mut counter = 0;
    let limit = 5;
    for node in graph.nodes() {
        if counter > limit {
            break;
        }

        println!("Node: {:?}", node);
        counter += 1;
    }

    println!("==========================================================");

    counter = 0;
    for edge in graph.edges() {
        if counter > limit {
            break;
        }

        println!("Edge: {:?}", edge);
        counter += 1;
    }
}

/// Read an edge list dataset from a text file and load it into a graph.
fn read_edge_list_dataset(
    path: &str,
    graph: &mut Graph<i32, f32>,
    sep: char,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = read_edge_list(path, graph, sep) {
        eprintln!("Error reading edges: {}", e);
    } else {
        println!(
            "Loaded graph from '{}' that has {} nodes and {} edges",
            path,
            graph.node_count(),
            graph.edge_count()
        );
    }
    Ok(())
}

/// Writes the adjacency list of a graph to a text file
fn write_adjacency_list_dataset(
    path: &str,
    graph: &Graph<i32, f32>,
    sep: char,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = write_adjacency_list(path, graph, sep) {
        eprintln!("Error writing adjacency list: {}", e);
    } else {
        println!("Wrote adjacency list to '{}'", path);
    }
    Ok(())
}

/// Write the edge list to a file
fn write_edge_list_dataset(
    path: &str,
    graph: &Graph<i32, f32>,
    sep: char,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = write_edge_list(path, graph, sep) {
        eprintln!("Error writing edge list: {}", e);
    } else {
        println!("Wrote edge list to '{}'", path);
    }
    Ok(())
}
