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

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> u64 {
    let op_re = Regex::new(r"[*+]\s*").unwrap();
    let mut sum = 0;
    op_re
        .find_iter(input.lines().last().unwrap())
        .for_each(|op| {
            let op_char = op.as_str().chars().nth(0).unwrap();
            let mut nums = vec![0; op.end() - op.start()];
            let mut offsets = vec![0; op.end() - op.start()];
            for (i, l) in input.lines().rev().skip(1).enumerate() {
                let digits = &l[op.start()..op.end()];
                for (j, c) in digits.chars().enumerate() {
                    if c != ' ' {
                        let digit: u64 = c.to_string().parse().unwrap();
                        nums[j] += digit * 10_u64.pow(i as u32 - offsets[j]);
                    } else {
                        offsets[j] += 1;
                    }
                }
            }
            let result = nums.into_iter().reduce(|acc, el| {
                if el != 0 {
                    return match op_char {
                        '+' => acc + el,
                        _ => acc * el
                    };
                }
                acc
            });
            sum += result.unwrap();
        });
    sum
}
