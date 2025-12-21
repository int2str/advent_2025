#![allow(dead_code)]

use std::collections::HashMap;
use std::io::Result;
use std::path::Path;

type NodeMap = HashMap<String, Vec<String>>;

fn read_reactor<P: AsRef<Path>>(path: P) -> Result<NodeMap> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let key = parts[0].trim().to_string();
            let values = parts[1].split_whitespace().map(|s| s.to_string()).collect();
            (key, values)
        })
        .collect())
}

fn trace_node(node_map: &NodeMap, search: &str) -> usize {
    if search == "out" {
        1
    } else {
        node_map[search]
            .iter()
            .map(|next| trace_node(node_map, next))
            .sum()
    }
}

fn trace_fft_dac(
    node_map: &NodeMap,
    search: &str,
    mut seen_fft: bool,
    mut seen_dac: bool,
    cache: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    let key = (search.to_string(), seen_fft, seen_dac);
    if let Some(&result) = cache.get(&key) {
        return result;
    }

    match search {
        "dac" => seen_dac = true,
        "fft" => seen_fft = true,
        _ => {}
    }

    let result = match search {
        "out" => (seen_fft && seen_dac) as usize,
        _ => node_map[search]
            .iter()
            .map(|next| trace_fft_dac(node_map, next, seen_fft, seen_dac, cache))
            .sum(),
    };

    cache.insert(key, result);
    result
}

fn trace_svr(node_map: &NodeMap) -> usize {
    let mut cache = HashMap::new();
    trace_fft_dac(node_map, "svr", false, false, &mut cache)
}

fn main() {
    println!("Run 'cargo test' instead ;)");
}

#[cfg(test)]
mod tests {
    mod day11_reactor {
        use super::super::*;

        #[test]
        fn sample() {
            let node_map = read_reactor("data/sample.txt").unwrap();
            assert_eq!(trace_node(&node_map, "you"), 5);

            let node_map2 = read_reactor("data/sample2.txt").unwrap();
            assert_eq!(trace_svr(&node_map2), 2);
        }

        #[test]
        #[cfg(feature = "private")]
        fn input() {
            let node_map = read_reactor("data/input.txt").unwrap();
            assert_eq!(trace_node(&node_map, "you"), 500);
            assert_eq!(trace_svr(&node_map), 287039700129600);
        }
    }
}
