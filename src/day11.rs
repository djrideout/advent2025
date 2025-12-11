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

fn paths(node: String, path: Vec<&str>, graph: &Input, cache: &mut HashMap<String, usize>) -> usize {
    if cache.contains_key(&node) {
        return *cache.get(&node).unwrap();
    }
    let mut count = 0;
    let children = graph.get(&node).unwrap();
    for child in children {
        let mut next_path = path.clone();
        next_path.push(child);
        if path[0] == "svr" && child == "out" {
            if path.contains(&"fft") && path.contains(&"dac") {
                return 1;
            }
            return 0;
        } else if child == "out" {
            return 1;
        }
        count += paths(child.clone(), next_path, graph, cache);
    }
    cache.insert(node.clone(), count);
    count
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Input) -> usize {
    paths("you".to_string(), vec!["you"], input, &mut HashMap::new())
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Input) -> usize {
    paths("svr".to_string(), vec!["svr"], input, &mut HashMap::new())
}
