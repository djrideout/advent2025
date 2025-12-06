use regex::Regex;

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let op_re = Regex::new(r"[*+]").unwrap();
    let num_re = Regex::new(r"-?\d+").unwrap();
    let mut sum = 0;
    op_re
        .find_iter(input.lines().last().unwrap())
        .for_each(|op| {
            let mut result = match op.as_str() {
                "+" => 0,
                _ => 1
            };
            for l in input.lines().rev().skip(1) {
                let num: u64 = num_re
                    .find(l.split_at(op.start()).1)
                    .unwrap().as_str().parse().unwrap();
                result = match op.as_str() {
                    "+" => result + num,
                    _ => result * num
                }
            }
            sum += result;
        });
    sum
}
