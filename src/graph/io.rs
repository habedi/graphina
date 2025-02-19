use crate::graph::types::{GraphConstructor, GraphWrapper};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Result, Write};

pub fn read_edge_list<Ty>(
    path: &str,
    graph: &mut GraphWrapper<i32, f32, Ty>,
    sep: char,
) -> Result<()>
where
    Ty: GraphConstructor<i32, f32>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut node_map = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let tokens: Vec<&str> = line.trim().split(sep).map(|s| s.trim()).collect();
        if tokens.len() < 2 {
            continue;
        }
        let src_val: i32 = tokens[0].parse().unwrap();
        let tgt_val: i32 = tokens[1].parse().unwrap();
        let weight: f32 = if tokens.len() >= 3 {
            tokens[2].parse().unwrap()
        } else {
            1.0
        };
        let src_node = *node_map
            .entry(src_val)
            .or_insert_with(|| graph.add_node(src_val));
        let tgt_node = *node_map
            .entry(tgt_val)
            .or_insert_with(|| graph.add_node(tgt_val));
        graph.add_edge(src_node, tgt_node, weight);
    }
    Ok(())
}

pub fn write_edge_list<Ty>(path: &str, graph: &GraphWrapper<i32, f32, Ty>, sep: char) -> Result<()>
where
    Ty: GraphConstructor<i32, f32>,
{
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for (src, tgt, weight) in graph.edges() {
        let src_attr = graph.node_attr(src).unwrap();
        let tgt_attr = graph.node_attr(tgt).unwrap();
        writeln!(writer, "{}{}{}{}{}", src_attr, sep, tgt_attr, sep, weight)?;
    }
    writer.flush()?;
    Ok(())
}
