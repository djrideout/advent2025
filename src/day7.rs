use std::collections::HashMap;

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> usize {
    let start = input.find("S").unwrap();
    let mut beams = vec![start];
    let mut count = 0;
    for l in input.lines().skip(2) {
        let splitters = l.match_indices("^");
        for s in splitters {
            let check_hit = beams.iter().enumerate().find(|(_, b)| **b == s.0);
            if let Some((i, hit)) = check_hit {
                count += 1;
                beams.append(&mut vec![*hit - 1, *hit + 1]);
                beams.remove(i);
            }
        }
        beams.sort();
        beams.dedup();
    }
    count
}

fn split(input: &str, line_num: usize, pos: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    let key = (line_num, pos);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }
    let l = input.lines().nth(line_num);
    if l.is_none() {
        return 1;
    }
    if l.unwrap().chars().nth(pos).unwrap() != '^' {
        return split(input, line_num + 2, pos, cache);
    }
    let timelines = split(input, line_num + 2, pos - 1, cache)
        + split(input, line_num + 2, pos + 1, cache);
    cache.insert(key, timelines);
    timelines
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> usize {
    let start = input.find("S").unwrap();
    split(input, 2, start, &mut HashMap::new())
}
