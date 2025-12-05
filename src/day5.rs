

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut fresh_ids = vec![];
    let mut available_ids = vec![];
    let mut parsing_available = false;
    for line in input.lines() {
        if parsing_available {
            let id: u64 = line.parse().unwrap();
            available_ids.push(id);
            continue;
        }
        if line.is_empty() {
            parsing_available = true;
            continue;
        }
        let (start, end) = line.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        fresh_ids.push((start, end));
    }
    fresh_ids.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged_fresh_ids = vec![];
    merged_fresh_ids.push(fresh_ids[0]);
    for (start, end) in fresh_ids.iter().skip(1) {
        let (_, last_end) = merged_fresh_ids.last_mut().unwrap();
        if start <= last_end {
            if end > last_end {
                *last_end = *end;
            }
        } else {
            merged_fresh_ids.push((*start, *end));
        }
    }
    (merged_fresh_ids, available_ids)
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &(Vec<(u64, u64)>, Vec<u64>)) -> u64 {
    let mut fresh_count = 0;
    for id in &input.1 {
        for (start, end) in &input.0 {
            if id >= start && id <= end {
                fresh_count += 1;
                break;
            }
        }
    }
    fresh_count
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &(Vec<(u64, u64)>, Vec<u64>)) -> u64 {
    input.0.iter().fold(0, |acc, (start, end)| acc + end - start + 1)
}
