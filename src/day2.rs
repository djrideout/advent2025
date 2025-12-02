fn sum_invalid(input: &str, only_two: bool) -> i64 {
    let mut sum = 0;
    input
        .split(",")
        .for_each(|range| {
            let mut parts = range.split("-");
            let lo = parts.next().unwrap().parse::<i64>().unwrap();
            let hi = parts.next().unwrap().parse::<i64>().unwrap();
            for n in lo..=hi {
                let s = n.to_string();
                let max_chunks = match only_two {
                    true => 2,
                    false => s.len()
                };
                let mut num_chunks = 2;
                while num_chunks <= max_chunks {
                    let chunk_len = s.len() / num_chunks;
                    if only_two && s.len() % 2 != 0 {
                        break;
                    }
                    let chunks: Vec<String> = s.chars()
                        .collect::<Vec<char>>()
                        .chunks(chunk_len)
                        .map(|chunk| chunk.iter().collect::<String>())
                        .collect();
                    if chunks.iter().all(|x| x == &chunks[0]) {
                        sum += n;
                        break;
                    }
                    num_chunks += 1;
                }
            }
        });
    sum
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> i64 {
    sum_invalid(input, true)
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> i64 {
    sum_invalid(input, false)
}
