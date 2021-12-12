use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;

fn num_paths(node: &str, mut path: HashSet<String>, graph: &HashMap<String, Vec<String>>) -> usize {
    if path.contains(node) && node == node.to_ascii_lowercase() {
        return 0;
    }

    if node == "end" {
        return 1;
    }

    path.insert(String::from(node));
    graph
        .get(node)
        .unwrap()
        .iter()
        .map(|adj| num_paths(adj, path.clone(), graph))
        .sum()
}

fn main() {
    let mut graph = HashMap::new();
    for (u, v) in std::io::stdin().lock().lines().flat_map(|line| {
        let line_str = line.unwrap();
        let (u, v) = line_str.split_once('-').unwrap();
        [
            (String::from(u), String::from(v)),
            (String::from(v), String::from(u)),
        ]
    }) {
        graph.entry(u).or_insert_with(Vec::new).push(v);
    }

    println!("{}", num_paths("start", HashSet::new(), &graph));
}
