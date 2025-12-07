use std::collections::HashMap;

fn split(input: &str, line_num: usize, pos: usize, split_count: &mut usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    let key = (line_num, pos);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }
    let l = input.lines().nth(line_num);
    if l.is_none() {
        return 1;
    }
    if l.unwrap().chars().nth(pos).unwrap() != '^' {
        return split(input, line_num + 2, pos, split_count, cache);
    }
    let timelines = split(input, line_num + 2, pos - 1, split_count, cache)
        + split(input, line_num + 2, pos + 1, split_count, cache);
    *split_count += 1;
    cache.insert(key, timelines);
    timelines
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> usize {
    let start = input.find("S").unwrap();
    let mut split_count = 0;
    split(input, 2, start, &mut split_count, &mut HashMap::new());
    split_count
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> usize {
    let start = input.find("S").unwrap();
    split(input, 2, start, &mut 0, &mut HashMap::new())
}
