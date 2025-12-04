use crate::grid::{Grid, Input, generate_input};

#[aoc_generator(day4)]
pub fn input_generator(in_lines: &str) -> Input {
    generate_input(in_lines)
}

fn count_rolls(input_str: &String, grid: &Grid) -> Vec<usize> {
    let mut remove_positions = vec![];
    for (i, char) in input_str.chars().enumerate() {
        if char != '@' {
            continue;
        }
        let mut adjacent_count = 0;
        for dir in 0..8 {
            let adjacent_char = grid.get_char(i as i32, dir, 1);
            if adjacent_char.0 == '@' {
                adjacent_count += 1;
            }
        }
        if adjacent_count < 4 {
            remove_positions.push(i);
        }
    }
    remove_positions
}

#[aoc(day4, part1)]
pub fn solve_part1(input_data: &Input) -> usize {
    let Input { input, line_len } = input_data;
    let grid = Grid::new(input.clone(), *line_len);
    count_rolls(input, &grid).len()
}

#[aoc(day4, part2)]
pub fn solve_part2(input_data: &Input) -> usize {
    let Input { input: init_input, line_len } = input_data;
    let mut input = init_input.clone();
    let mut grid = Grid::new(init_input.clone(), *line_len);
    let mut total_remove_count = 0;
    loop  {
        let remove_positions = count_rolls(&input, &grid);
        if remove_positions.len() == 0 {
            break;
        }
        total_remove_count += remove_positions.len();
        let mut next_input = input.clone().into_bytes();
        for i in remove_positions.iter() {
            next_input[*i] = b'.';
        }
        input = String::from_utf8(next_input).unwrap();
        grid = Grid::new(input.clone(), *line_len);
    }
    total_remove_count
}
