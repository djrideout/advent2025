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
