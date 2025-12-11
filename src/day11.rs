use std::collections::HashMap;

type Input = HashMap<String, Vec<String>>;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Input {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(':');
        let node = parts.next().unwrap().trim().to_string();
        let edges = parts
            .next()
            .unwrap_or("")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        graph.insert(node, edges);
    }
    graph
}

fn paths(node: String, path: Vec<&str>, graph: &Input, cache: &mut HashMap<String, (usize, usize, usize, usize)>) -> (usize, usize, usize, usize) {
    if cache.contains_key(&node) {
        return *cache.get(&node).unwrap();
    }
    let mut counts = (0, 0, 0, 0);
    let children = graph.get(&node).unwrap();
    for child in children {
        let mut next_path = path.clone();
        next_path.push(child);
        if child == "out" {
            return (1, 0, 0, 0);
        }
        let result = paths(child.clone(), next_path, graph, cache);
        counts.0 += result.0;
        counts.1 += result.1;
        counts.2 += result.2;
        counts.3 += result.3;
        if node == "dac" && result.2 > 0 {
            counts.3 += result.2;
        }
        if node == "fft" && result.1 > 0 {
            counts.3 += result.1;
        }
    }
    if node == "dac" {
        counts.1 += counts.0;
    }
    if node == "fft" {
        counts.2 += counts.0;
    }
    cache.insert(node.clone(), counts);
    counts
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Input) -> usize {
    paths("you".to_string(), vec!["you"], input, &mut HashMap::new()).0
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Input) -> usize {
    paths("svr".to_string(), vec!["svr"], input, &mut HashMap::new()).3
}
