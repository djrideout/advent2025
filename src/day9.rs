type Input = Vec<(i64, i64)>;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<(i64, i64)> {
    input.lines().map(|l| {
        let mut nums = l.split(',');
        (
            nums.nth(0).unwrap().parse::<i64>().unwrap(),
            nums.nth(0).unwrap().parse::<i64>().unwrap()
        )
    }).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Input) -> i64 {
    let mut max_area = 0;
    input
        .iter()
        .enumerate()
        .for_each(|(i, a)| {
            input.iter().skip(i + 1).for_each(|b| {
                let area = ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1);
                if area > max_area {
                    max_area = area;
                }
            });
        });
    max_area
}
