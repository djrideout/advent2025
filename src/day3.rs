fn joltage(input: &str, count: usize) -> u64 {
    let mut sum = 0;
    for l in input.lines() {
        let chars = l.chars().collect::<Vec<char>>();
        let mut digits: Vec<u64> = vec![0; count];
        let mut indices: Vec<usize> = vec![0; count + 1];
        let max_index = count - 1;
        for n in 0 ..= max_index {
            for i in indices[n] .. chars.len() - max_index + n {
                let d: u64 = chars[i].to_digit(10).unwrap().into();
                if d > digits[n] {
                    digits[n] = d;
                    indices[n + 1] = i + 1;
                    if digits[n] == 9 {
                        break;
                    }
                }
            }
        }
        sum += digits.iter().fold(0, |acc: u64, elem| acc * 10 + elem);
    }
    sum
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> u64 {
    joltage(input, 2)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> u64 {
    joltage(input, 12)
}
