fn count(input: &str, count_all: bool) -> i32 {
    let mut pos = 50;
    let mut count = 0;
    input
        .lines()
        .for_each(|l| {
            let (dir, num) = l.split_at(1);
            let dir = dir.chars().next().unwrap();
            let mut num: i32 = num.trim().parse().unwrap();
            if count_all {
                count += num / 100;
            }
            num = num % 100;
            let mut next = pos;
            let do_count = next != 0;
            if dir == 'L' {
                next -= num;
                if next <= 0 {
                    if do_count && count_all {
                        count += 1;
                    }
                    if next < 0 {
                        next = 100 + next;
                    }
                }
            } else {
                next += num;
                if next > 99 {
                    if do_count && count_all {
                        count += 1;
                    }
                    next = next - 100;
                }
            }
            if next == 0 && !count_all {
                count += 1;
            }
            pos = next;
        });
    count
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    count(input, false)
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i32 {
    count(input, true)
}
